const { HttpAgent, Actor } = require("@dfinity/agent");
const { IDL } = require("@dfinity/candid");
const { Ed25519KeyIdentity } = require("@dfinity/identity");

const fetch = require("cross-fetch");

const fs = require("fs");
const path = require("path");

const OUTPUT_DIR = "canisters";

const enviros = {
  ic: {
    name: "ic",
    url: "https://ic0.app",
  },
  local: {
    name: "local",
    url: "http://127.0.0.1:8000",
  },
};

const getKey = (keyfile) => fs.readFileSync(keyfile).toString();
const saveKey = (keyfile, data) => fs.writeFileSync(keyfile, data);

const getAuthKey = (keyPath) => {
  let key = null;
  if (fs.existsSync(keyPath)) {
    const found_key = getKey(keyPath);
    key = Ed25519KeyIdentity.fromJSON(found_key);
  } else {
    key = Ed25519KeyIdentity.generate();
    saveKey(keyPath, JSON.stringify(key));
  }
  console.log("Using identity:", key.getPrincipal().toText());
  return key;
};

const getActor = async (canisterName, canisterId, keyPath, host = "local") => {
  const enviro = enviros[host];
  const agent = new HttpAgent({
    fetch,
    identity: getAuthKey(keyPath),
    host: enviro.url,
  });
  await agent.fetchRootKey();
  const candid = eval(getCandid(canisterName));
  const actorConstructor = Actor.createActorClass(candid);

  globalThis.ic = {
    HttpAgent,
    IDL,
    canister: undefined,
    agent,
  };

  return new actorConstructor({
    agent,
    canisterId: canisterId,
  });
};

exports.getActor = getActor;

const getCanisterPath = (canisterName) =>
  path.join(__dirname, "..", ".dfx", "local", OUTPUT_DIR, canisterName);

const getCandid = (canisterName) =>
  fs
    .readFileSync(`${getCanisterPath(canisterName)}/${canisterName}.did.js`)
    .toString()
    .replace("export const idlFactory = ", "")
    .replace("export ", "")
    .replace("export ", "");
