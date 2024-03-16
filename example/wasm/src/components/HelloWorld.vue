<script setup lang="ts">
import { ref } from "vue";
import { Synthesizer, VoiceModel, OpenJtalkRc } from "../voicevoxCore";
import * as core from "../voicevoxCore";

const getVersion = async () => {
  version.value = await core.getVersion();
};

const modelLoad = async () => {
  const modelBlob = await fetch("/sample.vvm").then((res) => res.blob());
  const modelUint8Array = new Uint8Array(await modelBlob.arrayBuffer());
  const id = await VoiceModel.newFromPath(modelUint8Array);
  model.value = id;
  console.log(await model.value.metas());
};

const synthesizerCreate = async () => {
  const openJtalkRc = await OpenJtalkRc.new();
  const id = await Synthesizer.new(openJtalkRc);
  synthesizer.value = id;
};
const synthesizerLoadModel = async () => {
  if (model.value === undefined) {
    return;
  }
  if (synthesizer.value === undefined) {
    return;
  }
  await synthesizer.value.loadVoiceModel(model.value);
};
const synthesizerTts = async () => {
  if (synthesizer.value === undefined) {
    return;
  }
  const audio = await synthesizer.value.tts("ハローワールド", 0);
  audioSrc.value =
    "data:audio/wav;base64," +
    btoa(
      new Uint8Array(audio).reduce(
        (data, byte) => data + String.fromCharCode(byte),
        ""
      )
    );
};

const version = ref<string | undefined>(undefined);
const model = ref<VoiceModel | undefined>(undefined);
const synthesizer = ref<Synthesizer | undefined>(undefined);
const audioSrc = ref<string | undefined>(undefined);
</script>

<template>
  <div class="card">
    <div class="info">
      <div class="display">
        <div>Version</div>
        <pre>{{ version }}</pre>
      </div>
      <div class="display">
        <div>VoiceModel</div>
        <pre>{{ model }}</pre>
      </div>
      <div class="display">
        <div>Synthesizer</div>
        <pre>{{ synthesizer }}</pre>
      </div>
    </div>
    <audio controls :src="audioSrc"></audio>
    <button type="button" @click="getVersion">getVersion</button>
    <button type="button" @click="modelLoad">VoiceModel.load</button>
    <button type="button" @click="synthesizerCreate">Synthesizer.create</button>
    <button
      type="button"
      @click="synthesizerLoadModel"
      :disabled="model === undefined || synthesizer === undefined"
    >
      Synthesizer.loadModel
    </button>
    <button type="button" @click="synthesizerTts">Synthesizer.tts</button>
  </div>
</template>

<style scoped>
.info {
  display: flex;
  gap: 1rem;
}

.display {
  width: 320px;
  height: 240px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
  pre {
    text-align: left;
    overflow: scroll;
    border-radius: 8px;
    font-family: Consolas, monospace;
    background-color: #ccc;
    padding: 1rem;
    flex-grow: 1;
  }
}
.card {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
