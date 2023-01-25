# Murmelbahn

**NOTE**: This app is in no way affiliated with Ravensburger, it is not an official app.
Please do not contact Ravensburger if you have issues. Open an issue or discussion in this repository instead.

This app can be used to download Gravitrax courses and see information about them, if you know the codes used in the app.

There is a CLI, which is incomplete (and might be removed in the future) and a Web App, which is currently deployed at: https://murmelbahn.fly.dev

At the moment, only two endpoints are supported, and you need to call them manually by editing the URL:

- Dump: https://murmelbahn.fly.dev/course/GDZJZA3J3T/dump
- Bill of Material (BOM): https://murmelbahn.fly.dev/course/GDZJZA3J3T/bom
  - This one takes a query parameter called`format` with values of `csv`, `rust` or `json` (the default)
  - `csv` output is compatible with the GraviSheet

## Acknowledgements

- Thank you very much [Chris Fuchser](https://www.youtube.com/channel/UCk8bK1u_oH2LIGb_PLP7E9g) for all your help in understanding Gravitrax and testing this program
