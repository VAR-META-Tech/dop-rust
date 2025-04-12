// src/__tests__/engine.test.ts
import { describe, it, expect, beforeAll, afterAll } from 'vitest'
import * as fs from 'fs/promises';
import { getEngine, getProver, stopDopEngine } from 'dop-wallet-stagging'
import { initEngine, closeEngine } from '../engine.js'

describe('Dop Engine', () => {
    const ENGINE_TEST_DB = 'test.db';
    beforeAll(async () => {
        await fs.rm('test.db', { recursive: true, force: true }).catch(() => { })
        initEngine()
    })

    afterAll(async () => {
        await closeEngine()
    })

    it('Should get active engine instance', () => {
        const engine = getEngine();
        console.log(engine);
        expect(getEngine()).to.not.be.undefined;
    });

    it('Should fail without active engine instance', async () => {
        await stopDopEngine();
        expect(() => getEngine()).to.throw('DOP Engine not yet initialized.');
        expect(() => getProver()).to.throw('DOP Engine not yet initialized.');
    });
})
function rmDirSafe(ENGINE_TEST_DB: any) {
    throw new Error('Function not implemented.')
}

