use query::Query;
use answer::Answer;
use name_server::NameServer;

struct Resolver {
    name_servers: Vec<NameServer>
}

impl Resolver {
    pub fn resolve<T>(&self, domain_name: Query) -> Result<Answer<T>, &'static str> {
        Err("hogehoge")
    }
}
