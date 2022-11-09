const { Principal } = require("@dfinity/principal");
const dfinity = require("./dfinity");
const fs = require("fs");

var environment = "local";

if (args.length > 0) {
  environment = args[0];
}

const main = async () => {
  const actor = await dfinity.getActor(
    "soulbound_token",
    "cfnba-iiaaa-aaaab-qahoq-cai",
    "./keys/local-admin.json",
    environment
  );

  console.log(await actor.test());

  // console.log(
  //   "Minting token",
  //   await actor.mint(
  //     Principal.fromText(
  //       "uvcpu-lg3q2-maqwy-oxmdc-v5tja-nnz3m-hqpmz-jbkel-fg4rk-q3tsx-nae"
  //     )
  //   )
  // );

  let fileData = fs.readFileSync("./assets/SNS_TOKEN_IC_PERS_NOBG.png");
  console.log(await actor.add_asset(Uint8Array.from(fileData)));
};

main();
