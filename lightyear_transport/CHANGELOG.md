# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.21.0 (2025-07-03)

### Chore

 - <csr-id-7fe5e08d715fa55ad003270be95139b003aca396/> release 0.21.0

### Bug Fixes

 - <csr-id-97d5b9baf349aa8c0245d20432ff333c42b2c04d/> issues with prespawn match

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 1 unique issue was worked on: [#1064](https://github.com/cBournhonesque/lightyear/issues/1064)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1064](https://github.com/cBournhonesque/lightyear/issues/1064)**
    - Issues with prespawn match ([`97d5b9b`](https://github.com/cBournhonesque/lightyear/commit/97d5b9baf349aa8c0245d20432ff333c42b2c04d))
 * **Uncategorized**
    - Release 0.21.0 ([`7fe5e08`](https://github.com/cBournhonesque/lightyear/commit/7fe5e08d715fa55ad003270be95139b003aca396))
    - Adjusting changelogs prior to release of lightyear_serde v0.21.0, lightyear_utils v0.21.0, lightyear_core v0.21.0, lightyear_link v0.21.0, lightyear_aeronet v0.21.0, lightyear_connection v0.21.0, lightyear_macros v0.21.0, lightyear_transport v0.21.0, lightyear_messages v0.21.0, lightyear_replication v0.21.0, lightyear_sync v0.21.0, lightyear_interpolation v0.21.0, lightyear_prediction v0.21.0, lightyear_frame_interpolation v0.21.0, lightyear_avian2d v0.21.0, lightyear_avian3d v0.21.0, lightyear_crossbeam v0.21.0, lightyear_inputs v0.21.0, lightyear_inputs_bei v0.21.0, lightyear_inputs_leafwing v0.21.0, lightyear_inputs_native v0.21.0, lightyear_netcode v0.21.0, lightyear_steam v0.21.0, lightyear_webtransport v0.21.0, lightyear_udp v0.21.0, lightyear v0.21.0 ([`6ed9ae9`](https://github.com/cBournhonesque/lightyear/commit/6ed9ae95f9a75a9803c75c56c4e81f40f72fc3c8))
</details>

## v0.21.0-rc.3 (2025-07-03)

<csr-id-5dc2e81f8c2b1171df33703d73e38a49e7b4695d/>
<csr-id-81341e91707b31a5cba6967d23e230945180a4e8/>
<csr-id-f8d62c7389c6095694fcd8ceeb593b6d0ebc5b47/>
<csr-id-249b40f358977f6f85e269967d3912bfb4080f73/>
<csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/>

### Chore

 - <csr-id-5dc2e81f8c2b1171df33703d73e38a49e7b4695d/> release rc3
 - <csr-id-81341e91707b31a5cba6967d23e230945180a4e8/> release 0.21 rc 2
 - <csr-id-f8d62c7389c6095694fcd8ceeb593b6d0ebc5b47/> add more tests on replication internals, to prepare for fixing SinceLastSend
 - <csr-id-249b40f358977f6f85e269967d3912bfb4080f73/> fix clippy
 - <csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/> cargo fmt

### New Features

 - <csr-id-117b0841a25dba5c6ffaadad88a8c4dba09d3cbb/> support BEI inputs

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 14 commits contributed to the release over the course of 45 calendar days.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 8 unique issues were worked on: [#1017](https://github.com/cBournhonesque/lightyear/issues/1017), [#1021](https://github.com/cBournhonesque/lightyear/issues/1021), [#1023](https://github.com/cBournhonesque/lightyear/issues/1023), [#1039](https://github.com/cBournhonesque/lightyear/issues/1039), [#1043](https://github.com/cBournhonesque/lightyear/issues/1043), [#1054](https://github.com/cBournhonesque/lightyear/issues/1054), [#1055](https://github.com/cBournhonesque/lightyear/issues/1055), [#989](https://github.com/cBournhonesque/lightyear/issues/989)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#1017](https://github.com/cBournhonesque/lightyear/issues/1017)**
    - Release 0.21 rc1 ([`dc0e61e`](https://github.com/cBournhonesque/lightyear/commit/dc0e61e06fe68309ed8cbfdcdfead633ad567537))
 * **[#1021](https://github.com/cBournhonesque/lightyear/issues/1021)**
    - Fix lobby example (without HostServer) and add protocolhash ([`0beb664`](https://github.com/cBournhonesque/lightyear/commit/0beb664f0161f73e4a53c06530ae139078ed8763))
 * **[#1023](https://github.com/cBournhonesque/lightyear/issues/1023)**
    - Add HostServer ([`5b6af7e`](https://github.com/cBournhonesque/lightyear/commit/5b6af7edd3b41c05333d14dde258ea5e89c07c2d))
 * **[#1039](https://github.com/cBournhonesque/lightyear/issues/1039)**
    - Support BEI inputs ([`117b084`](https://github.com/cBournhonesque/lightyear/commit/117b0841a25dba5c6ffaadad88a8c4dba09d3cbb))
 * **[#1043](https://github.com/cBournhonesque/lightyear/issues/1043)**
    - Make workspace crates depend on individual bevy crates ([`5dc3dc3`](https://github.com/cBournhonesque/lightyear/commit/5dc3dc3e17a8b821c35162b904b73eea0e1c69be))
 * **[#1054](https://github.com/cBournhonesque/lightyear/issues/1054)**
    - Chore(docs) ([`59b9f7e`](https://github.com/cBournhonesque/lightyear/commit/59b9f7eb37b036488d3ceab780074274074a9bd6))
 * **[#1055](https://github.com/cBournhonesque/lightyear/issues/1055)**
    - Release 0.21 rc 2 ([`81341e9`](https://github.com/cBournhonesque/lightyear/commit/81341e91707b31a5cba6967d23e230945180a4e8))
 * **[#989](https://github.com/cBournhonesque/lightyear/issues/989)**
    - Bevy main refactor ([`b236123`](https://github.com/cBournhonesque/lightyear/commit/b236123c8331f9feea8c34cb9e0d6a179bb34918))
 * **Uncategorized**
    - Release lightyear_serde v0.21.0-rc.3, lightyear_utils v0.21.0-rc.3, lightyear_core v0.21.0-rc.3, lightyear_link v0.21.0-rc.3, lightyear_aeronet v0.21.0-rc.3, lightyear_connection v0.21.0-rc.3, lightyear_macros v0.21.0-rc.3, lightyear_transport v0.21.0-rc.3, lightyear_messages v0.21.0-rc.3, lightyear_replication v0.21.0-rc.3, lightyear_sync v0.21.0-rc.3, lightyear_interpolation v0.21.0-rc.3, lightyear_prediction v0.21.0-rc.3, lightyear_frame_interpolation v0.21.0-rc.3, lightyear_avian2d v0.21.0-rc.3, lightyear_avian3d v0.21.0-rc.3, lightyear_crossbeam v0.21.0-rc.3, lightyear_inputs v0.21.0-rc.3, lightyear_inputs_bei v0.21.0-rc.3, lightyear_inputs_leafwing v0.21.0-rc.3, lightyear_inputs_native v0.21.0-rc.3, lightyear_netcode v0.21.0-rc.3, lightyear_steam v0.21.0-rc.3, lightyear_webtransport v0.21.0-rc.3, lightyear_udp v0.21.0-rc.3, lightyear v0.21.0-rc.3 ([`134306e`](https://github.com/cBournhonesque/lightyear/commit/134306eaf4e23d2f609c8a7c93adc3c55618ff11))
    - Release rc3 ([`5dc2e81`](https://github.com/cBournhonesque/lightyear/commit/5dc2e81f8c2b1171df33703d73e38a49e7b4695d))
    - Add more tests on replication internals, to prepare for fixing SinceLastSend ([`f8d62c7`](https://github.com/cBournhonesque/lightyear/commit/f8d62c7389c6095694fcd8ceeb593b6d0ebc5b47))
    - Fix clippy ([`249b40f`](https://github.com/cBournhonesque/lightyear/commit/249b40f358977f6f85e269967d3912bfb4080f73))
    - Cargo fmt ([`f55c117`](https://github.com/cBournhonesque/lightyear/commit/f55c117c1627368978d26c788efbcb2ddda1da01))
    - Clippy ([`04f11a1`](https://github.com/cBournhonesque/lightyear/commit/04f11a1e1e031ae96f54c29f2803abab32e9a12b))
</details>

## v0.21.0-rc.2 (2025-07-01)

<csr-id-cedab052a0f47cf91b15267b8d83eb87524a8f4d/>
<csr-id-f8d62c7389c6095694fcd8ceeb593b6d0ebc5b47/>
<csr-id-249b40f358977f6f85e269967d3912bfb4080f73/>
<csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/>

### Chore

 - <csr-id-cedab052a0f47cf91b15267b8d83eb87524a8f4d/> add release command to ci
 - <csr-id-f8d62c7389c6095694fcd8ceeb593b6d0ebc5b47/> add more tests on replication internals, to prepare for fixing SinceLastSend
 - <csr-id-249b40f358977f6f85e269967d3912bfb4080f73/> fix clippy
 - <csr-id-f55c117c1627368978d26c788efbcb2ddda1da01/> cargo fmt

### New Features

 - <csr-id-117b0841a25dba5c6ffaadad88a8c4dba09d3cbb/> support BEI inputs

## v0.21.0-rc.1 (2025-06-08)

<csr-id-f361b72d433086c61ed6b4776fd4ee308c3747e1/>

### Chore

 - <csr-id-f361b72d433086c61ed6b4776fd4ee308c3747e1/> add changelogs

