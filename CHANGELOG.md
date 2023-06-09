# Changelog

All notable changes to this project will be documented in this file.

The format is (loosely) based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Do not change it, it is updated automatically.

## [unreleased]

### 🌟 Features


`murmur`:


- Start an application for recording and transcribing audio ([#31](https://github.com/schneiderfelipe/getanswe.rs/issues/31)) ([b8836d9](https://github.com/schneiderfelipe/getanswe.rs/commit/b8836d922aeb1cbdf978898306f0abd8ad831754))


### 🏗️ Miscellaneous Tasks


`deps`:


- Update async-openai requirement from 0.9.4 to 0.10.0 ([#32](https://github.com/schneiderfelipe/getanswe.rs/issues/32)) ([e054b8e](https://github.com/schneiderfelipe/getanswe.rs/commit/e054b8efee3d453e212a281ae8d83823a4646b7c))
- Update pretty_env_logger requirement from 0.4.0 to 0.5.0 ([#34](https://github.com/schneiderfelipe/getanswe.rs/issues/34)) ([e831928](https://github.com/schneiderfelipe/getanswe.rs/commit/e831928f466f00aae1c58955c6eb012397f07fd9))
- Update async-openai requirement from 0.10.3 to 0.11.0 ([#35](https://github.com/schneiderfelipe/getanswe.rs/issues/35)) ([7e3a760](https://github.com/schneiderfelipe/getanswe.rs/commit/7e3a760de045793f7dae03adff73d8d9c25bf5fd))
- Update rustyline requirement from 11.0.0 to 12.0.0 ([#37](https://github.com/schneiderfelipe/getanswe.rs/issues/37)) ([020d832](https://github.com/schneiderfelipe/getanswe.rs/commit/020d832aac45591070662c1a14e74c2c1d4840ef))
- Update async-openai requirement from 0.11.0 to 0.12.0 ([#36](https://github.com/schneiderfelipe/getanswe.rs/issues/36)) ([24a02a2](https://github.com/schneiderfelipe/getanswe.rs/commit/24a02a2f456a461bbb0ec48aca091f2e33384f05))


## [reply-v0.0.1-beta.2] - 2023-03-18

[9f85147](https://github.com/schneiderfelipe/getanswe.rs/commit/9f85147987d1e9d8fe730df661607744c43413be)...[477c153](https://github.com/schneiderfelipe/getanswe.rs/commit/477c1531bb83d78c866e48326d2c8d3af7ff6f1e)

### 🐛 Bug Fixes


[`reply`](https://github.com/schneiderfelipe/getanswe.rs/tree/main/reply#reply) ([crates.io](https://crates.io/crates/reply)):


- Add a new line to avoid the prompt to hide output ([#29](https://github.com/schneiderfelipe/getanswe.rs/issues/29)) ([c414e60](https://github.com/schneiderfelipe/getanswe.rs/commit/c414e6092c159869c195d8be1956443cc3d3a191))


## [reply-v0.0.1-beta.1] - 2023-03-17

[de9235a](https://github.com/schneiderfelipe/getanswe.rs/commit/de9235a59e5e14a0e7d90e353acf31a27a87571a)...[9f85147](https://github.com/schneiderfelipe/getanswe.rs/commit/9f85147987d1e9d8fe730df661607744c43413be)

### 🌟 Features


[`reply`](https://github.com/schneiderfelipe/getanswe.rs/tree/main/reply#reply) ([crates.io](https://crates.io/crates/reply)):


- Implement a minimum set of features ([#20](https://github.com/schneiderfelipe/getanswe.rs/issues/20)) ([67e3830](https://github.com/schneiderfelipe/getanswe.rs/commit/67e3830bfbe0e4d29f23c0dc817a75e6df0f68ff))
- Implement a minimum set of features ([#26](https://github.com/schneiderfelipe/getanswe.rs/issues/26)) ([34abb85](https://github.com/schneiderfelipe/getanswe.rs/commit/34abb85dacf7248b718274d420798b4df396d756))


### 🏗️ Miscellaneous Tasks


`README.md`:


- Change the emoji position and add link ([#24](https://github.com/schneiderfelipe/getanswe.rs/issues/24)) ([762c63e](https://github.com/schneiderfelipe/getanswe.rs/commit/762c63ecf277e29251d49f5132173bff92cf2961))


### 📝 Documentation


`README.md`:


- Add other badges ([#18](https://github.com/schneiderfelipe/getanswe.rs/issues/18)) ([c62cfd2](https://github.com/schneiderfelipe/getanswe.rs/commit/c62cfd2048c082b7ef5246de9eeb88c2f0458dd8))


### 🚀 Continuous Integration


`.github`:


- Correct transition performed in [#22](https://github.com/schneiderfelipe/getanswe.rs/issues/22) ([#23](https://github.com/schneiderfelipe/getanswe.rs/issues/23)) ([4c9ef4c](https://github.com/schneiderfelipe/getanswe.rs/commit/4c9ef4cfb91b4ed7e6b0241dcd029a95d176f84f))


## [answer-v0.0.1-beta.1] - 2023-03-15

[f64d1a1](https://github.com/schneiderfelipe/getanswe.rs/commit/f64d1a10038d64d9c96d30688164c285d5c773db)...[de9235a](https://github.com/schneiderfelipe/getanswe.rs/commit/de9235a59e5e14a0e7d90e353acf31a27a87571a)

### 🌟 Features


[`answer`](https://github.com/schneiderfelipe/getanswe.rs/tree/main/answer#answer) ([crates.io](https://crates.io/crates/answer)):


- Use async IO and multiple threads ([#13](https://github.com/schneiderfelipe/getanswe.rs/issues/13)) ([2b86a30](https://github.com/schneiderfelipe/getanswe.rs/commit/2b86a3037a14e1dc5c28ec2b7a41dc4b8548bd34))
- Flush as tokens are received ([#14](https://github.com/schneiderfelipe/getanswe.rs/issues/14)) ([0709b7b](https://github.com/schneiderfelipe/getanswe.rs/commit/0709b7b3b49484ee303aa8624d63bccbaf396c80))
- Improve ergonomics by using default features ([#16](https://github.com/schneiderfelipe/getanswe.rs/issues/16)) ([f6f0146](https://github.com/schneiderfelipe/getanswe.rs/commit/f6f01461ca005ddae16d0a250f67310cbb8e69ec))


### 🏗️ Miscellaneous Tasks


`changelog`:


- Improve CHANGELOG.md format ([#15](https://github.com/schneiderfelipe/getanswe.rs/issues/15)) ([b848284](https://github.com/schneiderfelipe/getanswe.rs/commit/b84828497bf84305001c03db9410b91ea0000b35))


`release`:


- Post first release adjustments ([#10](https://github.com/schneiderfelipe/getanswe.rs/issues/10)) ([6e2444f](https://github.com/schneiderfelipe/getanswe.rs/commit/6e2444f159fca322c51d37634c576c66d0e1541e))


### 📝 Documentation


`README.md`:


- Add instructions ([#11](https://github.com/schneiderfelipe/getanswe.rs/issues/11)) ([62143eb](https://github.com/schneiderfelipe/getanswe.rs/commit/62143ebe1d5808a9dd6c7034b622b3dfdafeba3f))


## [answer-v0.0.1-alpha.1] - 2023-03-14

### 🌟 Features


[`answer`](https://github.com/schneiderfelipe/getanswe.rs/tree/main/answer#answer) ([crates.io](https://crates.io/crates/answer)):


- Implement the minimal features ([#2](https://github.com/schneiderfelipe/getanswe.rs/issues/2)) ([498ac76](https://github.com/schneiderfelipe/getanswe.rs/commit/498ac76b41d6de3f73275f6926dc23a61d7088dc))


### 🏗️ Miscellaneous Tasks


`.github`:


- Setup repository ([#5](https://github.com/schneiderfelipe/getanswe.rs/issues/5)) ([90f77be](https://github.com/schneiderfelipe/getanswe.rs/commit/90f77be7db21e2acae86e470f92fddbaae5987fb))
- Create FUNDING.yml ([#3](https://github.com/schneiderfelipe/getanswe.rs/issues/3)) ([7eab8ca](https://github.com/schneiderfelipe/getanswe.rs/commit/7eab8ca4c5838126f9487c137abf86bbfbacbb72))


`README.md`:


- Add CI badges ([#7](https://github.com/schneiderfelipe/getanswe.rs/issues/7)) ([83e888c](https://github.com/schneiderfelipe/getanswe.rs/commit/83e888c994e3e3e50bac48fb7eac86e2ddb3d93d))


`cliff.toml`:


- Add config ([#6](https://github.com/schneiderfelipe/getanswe.rs/issues/6)) ([b91b883](https://github.com/schneiderfelipe/getanswe.rs/commit/b91b8838e287945f21f2a15a045c3333cf0dcb54))


### 📝 Documentation


`README.md`:


- Update description ([#1](https://github.com/schneiderfelipe/getanswe.rs/issues/1)) ([f97aede](https://github.com/schneiderfelipe/getanswe.rs/commit/f97aede8268fb0b4839a8d0e6b8679a70915d95b))


<!-- generated by git-cliff -->
