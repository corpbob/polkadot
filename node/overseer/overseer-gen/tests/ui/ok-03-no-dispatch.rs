#![allow(dead_code)]

use polkadot_overseer_gen::overlord;

#[derive(Default)]
struct AwesomeSubSys;

struct SigSigSig;

struct Event;

#[derive(Clone)]
struct MsgStrukt(u8);

#[overlord(signal=SigSigSig, event=Event, gen=AllMessages)]
struct Overseer {
	#[subsystem(no_dispatch, MsgStrukt)]
	sub0: AwesomeSubSys,

	i_like_pie: f64,
}

#[derive(Debug, Clone)]
struct DummySpawner;

struct DummyCtx;

fn main() {
	let overseer = Overseer::<_,_>::builder()
		.sub0(AwesomeSubSys::default())
		.i_like_pie(std::f64::consts::PI)
		.spawner(DummySpawner)
		.build(|| -> DummyCtx { DummyCtx } );
}