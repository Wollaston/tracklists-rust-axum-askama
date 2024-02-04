#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: uuid::Uuid,
}

//  Constructor
impl Ctx {
    pub fn new(user_id: uuid::Uuid) -> Ctx {
        Ctx { user_id }
    }

    pub fn user_id(&self) -> uuid::Uuid {
        self.user_id
    }
}
