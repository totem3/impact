use message::Message;
use answer::Answer;
use name_server::NameServer;

struct Resolver {
    name_servers: Vec<NameServer>
}
