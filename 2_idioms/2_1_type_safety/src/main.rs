mod post {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);

    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
}
mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
}

#[derive(Clone)]
struct NewPost {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

#[derive(Clone)]
struct UnmoderatedPost {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

#[derive(Clone)]
struct PublishedPost {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}


#[derive(Clone)]
struct DeletedPost {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

impl NewPost {
    fn publish(self) -> UnmoderatedPost {
        UnmoderatedPost {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl UnmoderatedPost {
    fn allow(self) -> PublishedPost {
        PublishedPost {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }

    fn deny(self) -> DeletedPost {
        DeletedPost {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}

impl PublishedPost {
    fn delete(self) -> DeletedPost {
        DeletedPost {
            id: self.id,
            user_id: self.user_id,
            title: self.title,
            body: self.body,
        }
    }
}



fn main() {
    println!("Implement me!");
    let new_post = NewPost {
        id: post::Id(1),
        user_id: user::Id(1),
        title: post::Title("Hello World".to_string()),
        body: post::Body("This is my first post".to_string()),
    };
    new_post.publish().allow().delete();
}
