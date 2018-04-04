use futures::future;
use futures::{Future, Stream};

use mercury_common::*;
use super::*;



pub struct HomeServer
{
    // TODO
}



impl HomeServer
{
    pub fn new() -> Self
        { Self {} }
}



impl ProfileRepo for HomeServer
{
    fn list(&self, /* TODO what filter criteria should we have here? */ ) ->
        Box< Stream<Item=Profile, Error=ErrorToBeSpecified> >
    {
        let (send, receive) = futures::sync::mpsc::channel(0);
        Box::new(receive.map_err( |_| ErrorToBeSpecified::TODO ) )
    }

    fn load(&self, id: &ProfileId) ->
        Box< Future<Item=Profile, Error=ErrorToBeSpecified> >
    {
        // Box::new( future::err(ErrorToBeSpecified::TODO) )
        let profile = Profile::new(
            &ProfileId( "Dummy ProfileId for testing load()".as_bytes().to_owned() ),
            &PublicKey( "Dummy PublicKey for testing load()".as_bytes().to_owned() ),
            &[] );
        Box::new( future::ok(profile) )
    }

    // NOTE should be more efficient than load(id) because URL is supposed to contain hints for resolution
    fn resolve(&self, url: &str) ->
        Box< Future<Item=Profile, Error=ErrorToBeSpecified> >
    {
        Box::new( future::err(ErrorToBeSpecified::TODO) )
    }
}



impl Home for HomeServer
{
    fn claim(&self, profile: ProfileId) ->
        Box< Future<Item=OwnProfile, Error=ErrorToBeSpecified> >
    {
        Box::new( futures::future::err(ErrorToBeSpecified::TODO) )
    }

    fn register(&mut self, own_prof: OwnProfile, invite: Option<HomeInvitation>) ->
        Box< Future<Item=OwnProfile, Error=(OwnProfile,ErrorToBeSpecified)> >
    {
        Box::new( futures::future::err( (own_prof,ErrorToBeSpecified::TODO) ) )
    }


    fn login(&self, profile: ProfileId) ->
        Box< Future<Item=Box<HomeSession>, Error=ErrorToBeSpecified> >
    {
        Box::new( futures::future::err(ErrorToBeSpecified::TODO) )
    }


    // NOTE acceptor must have this server as its home
    fn pair_request(&self, half_proof: RelationHalfProof) ->
        Box< Future<Item=(), Error=ErrorToBeSpecified> >
    {
        Box::new( futures::future::err(ErrorToBeSpecified::TODO) )
    }

    // NOTE acceptor must have this server as its home
    fn pair_response(&self, rel: RelationProof) ->
        Box< Future<Item=(), Error=ErrorToBeSpecified> >
    {
        Box::new( futures::future::err(ErrorToBeSpecified::TODO) )
    }

    fn call(&self, rel: RelationProof, app: ApplicationId, init_payload: AppMessageFrame) ->
        Box< Future<Item=CallMessages, Error=ErrorToBeSpecified> >
    {
        Box::new( futures::future::err(ErrorToBeSpecified::TODO) )
    }
}



pub struct HomeSessionServer
{
    // TODO
    // how to access context to get client profileId?
}


impl HomeSessionServer
{
    pub fn new() -> Self
        { Self{} }
}


impl HomeSession for HomeSessionServer
{
    fn update(&self, own_prof: &OwnProfile) ->
        Box< Future<Item=(), Error=ErrorToBeSpecified> >
    {
        Box::new( future::err(ErrorToBeSpecified::TODO) )
    }

    // NOTE newhome is a profile that contains at least one HomeFacet different than this home
    // TODO is the ID of the new home enough here or do we need the whole profile?
    fn unregister(&self, newhome: Option<Profile>) ->
        Box< Future<Item=(), Error=ErrorToBeSpecified> >
    {
        Box::new( future::err(ErrorToBeSpecified::TODO) )
    }


    fn events(&self) -> Box< Stream<Item=ProfileEvent, Error=ErrorToBeSpecified> >
    {
        let (sender, receiver) = futures::sync::mpsc::channel(0);
        Box::new( receiver.map_err( |_| ErrorToBeSpecified::TODO ) )
    }

    // TODO add argument in a later milestone, presence: Option<AppMessageFrame>) ->
    fn checkin_app(&self, app: &ApplicationId) ->
        Box< Stream<Item=Call, Error=ErrorToBeSpecified> >
    {
        let (sender, receiver) = futures::sync::mpsc::channel(0);
        Box::new( receiver.map_err( |_| ErrorToBeSpecified::TODO ) )
    }

    // TODO remove this after testing
    fn ping(&self, txt: &str) ->
        Box< Future<Item=String, Error=ErrorToBeSpecified> >
    {
        Box::new( future::ok( txt.to_owned() ) )
    }
}
