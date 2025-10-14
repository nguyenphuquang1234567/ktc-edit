<script lang="ts" context="module">
  export type Primitive = string | number | boolean | null;
  export type JSONValue = Primitive | { [key: string]: JSONValue } | JSONValue[];
</script>

<script lang="ts">
  import FieldEditor from "./FieldEditor.svelte";
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher<{ change: { path: string[]; value: JSONValue } }>();

  export let label: string | null = null;
  export let value: JSONValue;
  export let path: string[] = [];

  const isObject = (v: JSONValue): v is { [key: string]: JSONValue } =>
    typeof v === "object" && v !== null && !Array.isArray(v);

  $: inputId = (() => {
    const base = path.length > 0 ? path.join("-") : label ?? "root";
    return `field-${base.replace(/[^a-zA-Z0-9_-]/g, "-")}`;
  })();

  function forward(event: CustomEvent<{ path: string[]; value: JSONValue }>) {
    dispatch("change", event.detail);
  }

  function updateValue(newValue: JSONValue) {
    dispatch("change", { path, value: newValue });
  }

  function handleNumberInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement;
    if (target.value === "") {
      updateValue(null);
      return;
    }

    const parsed = Number(target.value);
    updateValue(Number.isNaN(parsed) ? 0 : parsed);
  }

  function handleTextInput(event: Event) {
    const target = event.currentTarget as HTMLInputElement | HTMLTextAreaElement;
    const raw = target.value;
    updateValue(raw.trim() === "null" ? null : raw);
  }
</script>

{#if Array.isArray(value)}
  <details open={path.length === 0} class="group">
    <summary>{label ?? "Tableau"} ({value.length})</summary>
    <div class="nested">
      {#each value as entry, index (index)}
        <FieldEditor
          label={`[${index}]`}
          value={entry}
          path={[...path, String(index)]}
          on:change={forward}
        />
      {/each}
    </div>
  </details>
{:else if isObject(value)}
  <details open={path.length === 0} class="group">
    <summary>{label ?? "Objet"}</summary>
    <div class="nested">
      {#each Object.entries(value) as [key, entry] (key)}
        <FieldEditor label={key} value={entry} path={[...path, key]} on:change={forward} />
      {/each}
    </div>
  </details>
{:else}
  <div class="field">
    {#if label}
      <label for={inputId}>{label}</label>
    {/if}
    {#if typeof value === "boolean"}
      <input
        id={inputId}
        type="checkbox"
        checked={value}
        on:change={(event) => updateValue((event.currentTarget as HTMLInputElement).checked)}
      />
    {:else if typeof value === "number"}
      <input id={inputId} type="number" value={value} on:input={handleNumberInput} />
    {:else}
      <input id={inputId} type="text" value={value ?? ""} on:input={handleTextInput} />
    {/if}
  </div>
{/if}

<style>
  .group {
    margin-bottom: 0.75rem;
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 10px;
    padding: 0.6rem 0.85rem;
    background: rgba(255, 255, 255, 0.82);
    box-shadow: 0 6px 16px rgba(15, 23, 42, 0.04);
  }

  details.group > summary {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-weight: 600;
    cursor: pointer;
    outline: none;
  }

  details.group > summary::-webkit-details-marker {
    display: none;
  }

  details.group > summary::after {
    content: "›";
    transition: transform 0.2s ease;
  }

  details.group[open] > summary::after {
    transform: rotate(90deg);
  }

  .nested {
    margin-top: 0.6rem;
    padding-left: 0.85rem;
    display: grid;
    gap: 0.55rem;
    border-left: 2px solid rgba(0, 0, 0, 0.06);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  label {
    font-weight: 600;
    font-size: 0.9rem;
  }

  input[type="text"],
  input[type="number"] {
    border-radius: 8px;
    border: 1px solid rgba(0, 0, 0, 0.2);
    padding: 0.45rem 0.65rem;
    font-size: 0.95rem;
    background: rgba(255, 255, 255, 0.95);
    color: inherit;
  }

  input[type="checkbox"] {
    width: 1.25rem;
    height: 1.25rem;
  }

  @media (prefers-color-scheme: dark) {
    .group {
      background: rgba(32, 32, 32, 0.85);
      border-color: rgba(255, 255, 255, 0.08);
      box-shadow: 0 8px 20px rgba(0, 0, 0, 0.45);
    }

    .nested {
      border-left-color: rgba(255, 255, 255, 0.1);
    }

    input[type="text"],
    input[type="number"] {
      background: rgba(24, 24, 24, 0.9);
      border-color: rgba(255, 255, 255, 0.12);
      color: #f3f3f3;
    }
  }
</style>
