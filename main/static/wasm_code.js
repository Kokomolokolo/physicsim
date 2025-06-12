let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const BallFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ball_free(ptr >>> 0, 1));

export class Ball {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BallFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ball_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get id() {
        const ret = wasm.__wbg_get_ball_id(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set id(arg0) {
        wasm.__wbg_set_ball_id(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get x() {
        const ret = wasm.__wbg_get_ball_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set x(arg0) {
        wasm.__wbg_set_ball_x(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get y() {
        const ret = wasm.__wbg_get_ball_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set y(arg0) {
        wasm.__wbg_set_ball_y(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get dx() {
        const ret = wasm.__wbg_get_ball_dx(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set dx(arg0) {
        wasm.__wbg_set_ball_dx(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get dy() {
        const ret = wasm.__wbg_get_ball_dy(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set dy(arg0) {
        wasm.__wbg_set_ball_dy(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get radius() {
        const ret = wasm.__wbg_get_ball_radius(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set radius(arg0) {
        wasm.__wbg_set_ball_radius(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get mass() {
        const ret = wasm.__wbg_get_ball_mass(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set mass(arg0) {
        wasm.__wbg_set_ball_mass(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get r() {
        const ret = wasm.__wbg_get_ball_r(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set r(arg0) {
        wasm.__wbg_set_ball_r(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get g() {
        const ret = wasm.__wbg_get_ball_g(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set g(arg0) {
        wasm.__wbg_set_ball_g(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get b() {
        const ret = wasm.__wbg_get_ball_b(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set b(arg0) {
        wasm.__wbg_set_ball_b(this.__wbg_ptr, arg0);
    }
}

const BallManagerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ballmanager_free(ptr >>> 0, 1));

export class BallManager {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(BallManager.prototype);
        obj.__wbg_ptr = ptr;
        BallManagerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BallManagerFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ballmanager_free(ptr, 0);
    }
    /**
     * @param {number} canvas_width
     * @param {number} canvas_height
     * @returns {BallManager}
     */
    static new(canvas_width, canvas_height) {
        const ret = wasm.ballmanager_new(canvas_width, canvas_height);
        return BallManager.__wrap(ret);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} dx
     * @param {number} dy
     * @param {number} radius
     * @param {number} mass
     * @param {number} r
     * @param {number} g
     * @param {number} b
     * @returns {number}
     */
    add_ball(x, y, dx, dy, radius, mass, r, g, b) {
        const ret = wasm.ballmanager_add_ball(this.__wbg_ptr, x, y, dx, dy, radius, mass, r, g, b);
        return ret >>> 0;
    }
    /**
     * @returns {Float32Array}
     */
    update_and_get_positions() {
        const ret = wasm.ballmanager_update_and_get_positions(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Ball} ball1
     * @param {Ball} ball2
     * @param {number} elasticity
     */
    static handle_colisons_between_balls_v1(ball1, ball2, elasticity) {
        _assertClass(ball1, Ball);
        _assertClass(ball2, Ball);
        wasm.ballmanager_handle_colisons_between_balls_v1(ball1.__wbg_ptr, ball2.__wbg_ptr, elasticity);
    }
    /**
     * @param {Ball} ball1
     * @param {Ball} ball2
     */
    static handle_colisons_between_balls_v2(ball1, ball2) {
        _assertClass(ball1, Ball);
        _assertClass(ball2, Ball);
        wasm.ballmanager_handle_colisons_between_balls_v2(ball1.__wbg_ptr, ball2.__wbg_ptr);
    }
    /**
     * @param {number} g
     */
    set_gravity(g) {
        wasm.ballmanager_set_gravity(this.__wbg_ptr, g);
    }
    /**
     * @returns {number}
     */
    get_gravity() {
        const ret = wasm.ballmanager_get_gravity(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} e
     */
    set_elasticity(e) {
        wasm.ballmanager_set_elasticity(this.__wbg_ptr, e);
    }
    /**
     * @returns {number}
     */
    get_elasticity() {
        const ret = wasm.ballmanager_get_elasticity(this.__wbg_ptr);
        return ret;
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
        const ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_new_780abee5c1739fd7 = function(arg0) {
        const ret = new Float32Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_e6b7e69acd4c7354 = function(arg0, arg1, arg2) {
        const ret = new Float32Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_0;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('wasm_code_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
