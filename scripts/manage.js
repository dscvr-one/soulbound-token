const { Principal } = require("@dfinity/principal");
const dfinity = require("./dfinity");

const main = async () => {
  const actor = await dfinity.getActor(
    "soulbound_token",
    "rkp4c-7iaaa-aaaaa-aaaca-cai",
    "./keys/local-admin.json",
    "local"
  );

  console.log(await actor.test());

  console.log(
    await actor.mint(
      Principal.fromText(
        "uvcpu-lg3q2-maqwy-oxmdc-v5tja-nnz3m-hqpmz-jbkel-fg4rk-q3tsx-nae"
      )
    )
  );
};

main();
