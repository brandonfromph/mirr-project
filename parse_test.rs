fn main() {
    let src = "module test {\nsignal a: in bool;\nsignal b: in bool;\nsignal c: in bool;\nassert p1: always a;\nassert p2: always b;\nassert p3: eventually within 5 cycles c;\n}";
    let mut reg = mirrc::ecs::Registry::new();
    mirrc::parser::ecs_parser::parse_mirr_ecs_with_base_dir(&mut reg, src, None).unwrap();
    for prop in reg.property_comps.iter().flatten() {
        println!("{:?}", prop.directive);
    }
    println!("Total props: {}", reg.property_comps.iter().flatten().count());
}
