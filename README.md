# Murmelbahn

**NOTE**: This app is in no way affiliated with Ravensburger, it is not an official app.
Please do not contact Ravensburger if you have issues. Open an issue or discussion in this repository instead.

This app can be used to download Gravitrax courses and see information about them, if you know the codes used in the app.
It is a web application, currently deployed at: https://murmelbahn.fly.dev

Some URLs that are supported:

- Dump: https://murmelbahn.fly.dev/course/GDZJZA3J3T/dump
- Bill of Material (BOM): https://murmelbahn.fly.dev/course/GDZJZA3J3T/bom
  - This one takes a query parameter called`format` with values of `csv`, `rust` or `json` (the default)
  - `csv` output is compatible with the GraviSheet
- Raw data: https://murmelbahn.fly.dev/course/GDZJZA3J3T/raw
  - This is the data as it comes from the Ravensburger API (only base64 decoded)
                   
## Course format

I used the [ImHex](https://github.com/WerWolv/ImHex) editor to reverse engineer the data format.
I did include the schema so you can look at files yourself if you like.
It can be found in the file `imhex-schema.txt` in this repository.
Ravensburger changed the file format multiple times.
This app can only process the latest three formats (anything after 2020, the introduction of "Pro").
Earlier versions could - in theory - be supported but I didn't find many tracks with those formats out there with the exception of courses between 2019 and 2020, I might add support for this later.

## Building & Running
  
Environment Variables:
```
DATABASE_URL=postgres://<user>:<password>:15432/murmelbahn;RUST_LOG=murmelbahn_web=debug,murmelbahn_lib=debug,warn;SETS_DIRECTORY=data/sets
```

```
flyctl proxy 15432:5433 -a murmelbahn-db -s
npm run build
cargo run --package murmelbahn-web --bin murmelbahn-web
```

Deploy: `flyctl deploy`

## Acknowledgements

- Thank you very much [Chris Fuchser](https://www.youtube.com/channel/UCk8bK1u_oH2LIGb_PLP7E9g) for all your help in understanding Gravitrax and testing this program
- [WerWolv](https://github.com/WerWolv/ImHex) for the fantastic ImHex editor. That was a fun week to play around with!
