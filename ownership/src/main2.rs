use std::ops::Drop;

#[derive(Debug)]
struct Parent(usize, Child, Child);

#[derive(Debug)]
struct Child(usize);

fn f1(p: &Parent)
{
    println!("p: {:?}", p);
}

fn f2(p: &mut Parent)
{
    p.0 += 1;
    println!("p: {:?}", p);
}

fn main()
{
    let mut p1 = Parent(1, Child(11), Child(12));
    f1(&p1);
    f2(&mut p1);
    println!("p1: {:?}", p1);
}

impl Drop for Parent
{
    fn drop(&mut self)
    {
        println!("Dropping {:?}", self);
    }
}

impl Drop for Child
{
    fn drop(&mut self)
    {
	println!("Droppping {:?}", self);
    }
}