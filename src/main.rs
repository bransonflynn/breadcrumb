pub mod crumbs;

fn main() {
    crumbs::put_crumbs(crumbs::Directory::Desktop);
    crumbs::put_crumbs(crumbs::Directory::Downloads);
}
