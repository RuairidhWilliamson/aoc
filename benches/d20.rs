use criterion::{criterion_group, criterion_main, Criterion};

pub fn push_button(c: &mut Criterion) {
    let input = "
%ls -> gl
%rz -> vm, gl
broadcaster -> rz, fp, kv, fd
%ql -> bn
%bm -> hr, fj
%fp -> cc, gk
&lk -> nc
%xg -> gl, mz
%dg -> gk, mp
%zg -> ls, gl
%lg -> hr
%pt -> lg, hr
%sp -> mj
%ms -> gl, hx
%kj -> fl, gk
%bn -> rj, gk
%xc -> vq
%fl -> gk
%dh -> hr, nm
%jk -> gk, dg
%tf -> cb
%kd -> cm, nr
&hr -> hh, kv, xl, qq
%kv -> xr, hr
%hq -> ql
&fn -> nc
%vm -> gl, xn
%jh -> nr, kd
%mz -> dd
%tp -> hq
%cf -> nr
%gr -> jh
%jd -> hr, bm
%xr -> qq, hr
%cm -> nr, cf
&fh -> nc
%rb -> xl, hr
&nc -> rx
%mp -> gk, kj
&nr -> fd, gr, fn, cb, tf, xc, vq
&gl -> fh, xn, sp, mz, rz, mj, dd
%rj -> jk
&hh -> nc
%fd -> nr, df
&gk -> lk, tp, fp, ql, hq, rj
%fj -> pt, hr
%qq -> dh
%df -> nr, nv
%mj -> ms
%xn -> xg
%cc -> gk, tp
%nm -> rb, hr
%dd -> sp
%vq -> gr
%cb -> xc
%nv -> tf, nr
%xl -> jd
%hx -> gl, zg
";
    let mut config: aoc::d20::Configuration = input.parse().unwrap();
    config.prepass();
    c.bench_function("push_button", |b| {
        // b.iter(|| config.push_button_check("rx", &aoc::d20::Pulse::Low))
    });
}

criterion_group!(benches, push_button);
criterion_main!(benches);
