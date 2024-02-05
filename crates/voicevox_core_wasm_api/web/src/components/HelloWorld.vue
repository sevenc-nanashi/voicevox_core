<script setup lang="ts">
import { ref } from "vue";
import { Synthesizer, Model } from "../voicevoxCore";
import * as core from "../voicevoxCore";

const getVersion = async () => {
  version.value = await core.getVersion();
};

const modelLoad = async () => {
  const modelBlob = await fetch("sample.vvm").then((res) => res.blob());
  const modelB64 = await new Promise<string>((resolve) => {
    const reader = new FileReader();
    reader.onload = () => {
      resolve((reader.result as string).split(",")[1]);
    };
    reader.readAsDataURL(modelBlob);
  });
  const id = await Model.load(modelB64);
  model.value = id;
};

const synthesizerCreate = async () => {
  const id = await Synthesizer.create();
  synthesizer.value = id;
};
const synthesizerLoadModel = async () => {
  if (model.value === undefined) {
    return;
  }
  if (synthesizer.value === undefined) {
    return;
  }
  await synthesizer.value.loadModel(model.value);
};
const synthesizerTts = async () => {
  if (synthesizer.value === undefined) {
    return;
  }
  const audio = await synthesizer.value.tts("ハローワールド");
  audioSrc.value = "data:audio/wav;base64," + audio;
};

const version = ref<string | undefined>(undefined);
const model = ref<Model | undefined>(undefined);
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
        <div>Model</div>
        <pre>{{ model }}</pre>
      </div>
      <div class="display">
        <div>Synthesizer</div>
        <pre>{{ synthesizer }}</pre>
      </div>
    </div>
    <audio controls :src="audioSrc"></audio>
    <button type="button" @click="getVersion">getVersion</button>
    <button type="button" @click="modelLoad">Model.load</button>
    <button type="button" @click="synthesizerCreate">Synthesizer.create</button>
    <button type="button" @click="synthesizerLoadModel">
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
