pub struct NewPost {
    pub content: String,
}

pub struct UnmoderatedPost {
    pub content: String,
}

pub struct PublishedPost {
    pub content: String,
}

pub struct DeletedPost {
    pub content: String,
}

impl NewPost {
    pub fn publish(self) -> UnmoderatedPost {
        UnmoderatedPost {
            content: self.content,
        }
    }
}

impl UnmoderatedPost {
    pub fn allow(self) -> PublishedPost {
        PublishedPost {
            content: self.content,
        }
    }

    pub fn deny(self) -> DeletedPost {
        DeletedPost {
            content: self.content,
        }
    }
}

impl PublishedPost {
    pub fn delete(self) -> DeletedPost {
        DeletedPost {
            content: self.content,
        }
    }
}

impl DeletedPost {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_typestate_flow() {
        let new = NewPost {
            content: "Hello".into(),
        };
        let unmod = new.publish();
        let published = unmod.allow();
        let deleted = published.delete();

        assert_eq!(deleted.content, "Hello");
    }
}
