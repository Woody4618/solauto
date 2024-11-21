import { describe, it } from "mocha";
import {
  ALL_SUPPORTED_TOKENS,
  TOKEN_INFO,
} from "../../src/constants/tokenConstants";
import {
  buildHeliusApiUrl,
  getSolanaRpcConnection,
} from "../../src/utils/solanaUtils";
import { publicKey } from "@metaplex-foundation/umi";
import { assert } from "chai";
import { getTokenAccount } from "../../src/utils";
import { SOLAUTO_FEES_WALLET } from "../../src/constants";
import { PublicKey } from "@solana/web3.js";

describe("Assert Solauto fee token accounts are created", async () => {
  it("all Solauto fee token accounts created", async () => {
    let [_, umi] = getSolanaRpcConnection(
      buildHeliusApiUrl(process.env.HELIUS_API_KEY!)
    );

    const tokenAccounts = await umi.rpc.getAccounts(
      ALL_SUPPORTED_TOKENS.map((x) =>
        publicKey(getTokenAccount(SOLAUTO_FEES_WALLET, new PublicKey(x)))
      )
    );
    for (let i = 0; i < tokenAccounts.length; i++) {
      console.log(tokenAccounts[i].publicKey.toString());
      if (!tokenAccounts[i].exists) {
        console.log(
          "Missing Solauto fees TA for ",
          TOKEN_INFO[ALL_SUPPORTED_TOKENS[i].toString()].ticker
        );
      }
    }
    assert(tokenAccounts.filter((x) => !x.exists).length === 0);
  });
});
