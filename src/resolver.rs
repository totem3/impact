use query::Query;
use answer::Answer;
use name_server::NameServer;

struct Resolver {
    name_servers: Vec<NameServer>
}

impl Resolver {
    pub fn resolve(&self, domain_name: Query) -> Result<Answer, &'static str> {
        Err("hogehoge")
    }
}
