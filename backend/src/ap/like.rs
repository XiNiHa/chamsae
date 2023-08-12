use activitypub_federation::{
    activity_queue::send_activity,
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::LikeType,
    protocol::{context::WithContext, verification::verify_domains_match},
    traits::{ActivityHandler, Object},
};
use async_trait::async_trait;
use sea_orm::ModelTrait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    entity::{post, reaction, user},
    error::{Context, Error},
    state::State,
};

use super::{person::LocalPerson, tag::Tag};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    #[serde(rename = "type")]
    pub ty: LikeType,
    pub id: ObjectId<reaction::Model>,
    pub actor: Url,
    pub object: ObjectId<post::Model>,
    pub content: String,
    #[serde(default)]
    pub tag: Vec<Tag>,
}

impl Like {
    #[tracing::instrument(skip(data))]
    pub async fn send(self, data: &Data<State>) -> Result<(), Error> {
        let post = self.object.dereference(data).await?;
        let user = post
            .find_related(user::Entity)
            .one(&*data.db)
            .await
            .context_internal_server_error("failed to query database")?
            .context_internal_server_error("user not found")?;
        let inbox =
            Url::parse(&user.inbox).context_internal_server_error("malformed user inbox URL")?;
        let with_context = WithContext::new_default(self);
        send_activity(with_context, &LocalPerson, vec![inbox], data).await
    }
}

#[async_trait]
impl ActivityHandler for Like {
    type DataType = State;
    type Error = Error;

    fn id(&self) -> &Url {
        self.id.inner()
    }

    fn actor(&self) -> &Url {
        &self.actor
    }

    #[tracing::instrument(skip(_data))]
    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        verify_domains_match(&self.actor, self.id.inner())
            .context_bad_request("failed to verify domain")
    }

    #[tracing::instrument(skip(data))]
    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        reaction::Model::from_json(self, data).await?;
        Ok(())
    }
}