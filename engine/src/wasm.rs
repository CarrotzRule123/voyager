struct WebAssemblyInstantiatedSource {
    required Module module;
    required Instance instance;
};


impl WebAssembly {
    boolean validate(BufferSource bytes);
    Promise<Module> compile(BufferSource bytes);

    Promise<WebAssemblyInstantiatedSource> instantiate(
        BufferSource bytes, optional object importObject);

    Promise<Instance> instantiate(
        Module moduleObject, optional object importObject);
};