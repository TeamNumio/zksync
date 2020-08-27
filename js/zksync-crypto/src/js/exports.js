// Copyright 2019-2020 @polkadot/wasm-crypto authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

/* eslint-disable sort-keys */

const { assert } = require('@polkadot/util');

const INIT_ERRROR = 'The WASM interface has not been initialized. Ensure that you wait for the initialization Promise with waitReady() from @polkadot/wasm-crypto (or cryptoWaitReady() from @polkadot/util-crypto) before attempting to use WASM-only interfaces.';

module.exports = function (stubbed) {
    const wrapReady = (fn) =>
        (...params) => {
            assert(stubbed.isReady(), INIT_ERRROR);

            return fn(...params);
        };

    return {
        zksync_crypto_init: wrapReady(stubbed.zksync_crypto_init),
        privateKeyFromSeed: wrapReady(stubbed.privateKeyFromSeed),
        private_key_to_pubkey_hash: wrapReady(stubbed.private_key_to_pubkey_hash),
        sign_musig: wrapReady(stubbed.sign_musig),
        __wbindgen_malloc: wrapReady(stubbed.__wbindgen_malloc),
        __wbindgen_free: wrapReady(stubbed.__wbindgen_free),
        __wbindgen_realloc: wrapReady(stubbed.__wbindgen_realloc),


        isReady: stubbed.isReady,
        waitReady: stubbed.waitReady
    }
}