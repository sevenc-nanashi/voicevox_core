let core: undefined | any = undefined;

declare const RawVoicevoxCore: () => Promise<any>;
RawVoicevoxCore().then(async (m: any) => {
  console.log("WASM loaded");
  core = m;
  await message("init", {});
  console.log("Initialized");
});

type Messages = {
  init: {};
  getVersion: {};
  modelLoad: {
    base64: string;
  };
  synthesizerCreate: {};
  synthesizerLoadModel: {
    synthesizer: number;
    model: number;
  };
  synthesizerTts: {
    synthesizer: number;
    text: string;
  };
};

async function message<T extends keyof Messages>(
  type: T,
  payload: Messages[T]
): Promise<string> {
  if (core === undefined) {
    throw new Error("core is not loaded");
  }
  console.log("Sending message:", type);
  const result = await core.ccall(
    "message",
    "string",
    ["string"],
    [
      JSON.stringify({
        type,
        payload,
      }),
    ]
  );
  console.log("Received.");
  return result;
}

export async function init(): Promise<string> {
  return message("init", {});
}

export async function getVersion(): Promise<string> {
  return message("getVersion", {});
}

export class Model {
  pointer: number;
  constructor(pointer: number) {
    this.pointer = pointer;
  }

  static async load(base64: string): Promise<Model> {
    const pointer = await message("modelLoad", { base64 });
    return new Model(parseInt(pointer));
  }
}

export class Synthesizer {
  pointer: number;
  constructor(pointer: number) {
    this.pointer = pointer;
  }

  static async create(): Promise<Synthesizer> {
    const pointer = await message("synthesizerCreate", {});
    return new Synthesizer(parseInt(pointer));
  }

  async loadModel(model: Model): Promise<void> {
    await message("synthesizerLoadModel", {
      synthesizer: this.pointer,
      model: model.pointer,
    });
  }

  async tts(text: string): Promise<string> {
    return message("synthesizerTts", {
      synthesizer: this.pointer,
      text,
    });
  }
}
