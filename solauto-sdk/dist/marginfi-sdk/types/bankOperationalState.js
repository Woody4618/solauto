"use strict";
/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.BankOperationalState = void 0;
exports.getBankOperationalStateSerializer = getBankOperationalStateSerializer;
const serializers_1 = require("@metaplex-foundation/umi/serializers");
var BankOperationalState;
(function (BankOperationalState) {
    BankOperationalState[BankOperationalState["Paused"] = 0] = "Paused";
    BankOperationalState[BankOperationalState["Operational"] = 1] = "Operational";
    BankOperationalState[BankOperationalState["ReduceOnly"] = 2] = "ReduceOnly";
})(BankOperationalState || (exports.BankOperationalState = BankOperationalState = {}));
function getBankOperationalStateSerializer() {
    return (0, serializers_1.scalarEnum)(BankOperationalState, {
        description: 'BankOperationalState',
    });
}