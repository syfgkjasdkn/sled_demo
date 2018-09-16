#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;
extern crate sled;

use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, Error, NifResult, Term};
use sled::{ConfigBuilder, Tree};
use std::sync::Mutex;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom nil;
    }
}

struct TreeResource(Mutex<Tree>);

rustler_export_nifs! {
    "Elixir.Sled.NIF",
    [
        ("open", 1, open),
        ("set", 3, set),
        ("cas", 4, cas),
        ("del", 2, del),
        ("get", 2, get)
    ],
    Some(load)
}

fn load(env: Env, _info: Term) -> bool {
    resource_struct_init!(TreeResource, env);
    true
}

fn open<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let path: String = try!(args[0].decode());

    let config = ConfigBuilder::new().path(path).build();
    let tree = Tree::start(config).unwrap();
    let resource = ResourceArc::new(TreeResource(Mutex::new(tree)));

    Ok((atoms::ok(), resource).encode(env))
}

fn set<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<TreeResource> = args[0].decode()?;
    let key: String = try!(args[1].decode());
    let value: String = try!(args[2].decode());

    let tree = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    tree.set(key.as_bytes().to_vec(), value.as_bytes().to_vec());

    Ok(atoms::ok().encode(env))
}

fn cas<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<TreeResource> = args[0].decode()?;
    let key: String = try!(args[1].decode());
    let v1: String = try!(args[2].decode());
    let v2: String = try!(args[3].decode());

    let tree = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    tree.cas(
        key.as_bytes().to_vec(),
        Some(v1.as_bytes().to_vec()),
        Some(v2.as_bytes().to_vec()),
    );

    Ok(atoms::ok().encode(env))
}

fn del<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<TreeResource> = args[0].decode()?;
    let key: String = try!(args[1].decode());

    let tree = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    tree.del(&key.as_bytes());

    Ok(atoms::ok().encode(env))
}

fn get<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<TreeResource> = args[0].decode()?;
    let key: String = try!(args[1].decode());

    let tree = match resource.0.try_lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::BadArg),
    };

    match tree.get(&key.as_bytes()) {
        Ok(Some(v)) => Ok(v.encode(env)),
        Ok(None) => Ok(atoms::nil().encode(env)),
        Err(_) => Ok(atoms::error().encode(env)),
    }
}
