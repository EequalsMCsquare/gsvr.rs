use super::DBComponent;
use crate::error::{Error, Result};
use mongodb::bson::{Binary, Document};
use std::result::Result as StdResult;

#[allow(non_snake_case)]
#[allow(unused)]
impl DBComponent {
    pub(super) async fn on_DBLoadReq(
        db: &mongodb::Database,
        coll: &str,
        filter: &Option<Document>,
    ) -> Result<Binary> {
        let res: StdResult<Option<Binary>, _> =
            db.collection(&coll).find_one(filter.clone(), None).await;
        match res {
            Ok(Some(bin)) => Ok(bin),
            Ok(None) => Err(Error::NoDBRecord {
                coll: coll.to_owned(),
            }),
            Err(err) => Err(Error::Database(err)),
        }
    }

    pub(super) async fn on_DBBulkUpsertReq(
        db: &mongodb::Database,
        coll: &str,
        data: Vec<Document>,
    ) -> Result<()> {
        Ok(())
    }
}
