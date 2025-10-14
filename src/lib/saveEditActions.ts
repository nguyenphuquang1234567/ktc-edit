import type { JSONValue } from "$lib/components/FieldEditor.svelte";

type MutableJSONObject = Record<string, any>;

interface SelectionOptions {
  campaignIndex: number;
  islandIndex: number;
  playerIndex: number;
}

export interface CoinsOptions extends SelectionOptions {
  coins: number;
}

export type CoinsQueryOptions = SelectionOptions;

export interface GemsOptions extends SelectionOptions {
  gems: number;
}

export type GemsQueryOptions = SelectionOptions;

export interface SpawnArchersOptions extends SelectionOptions {
  count: number;
}

function toObject(value: unknown, message: string): MutableJSONObject {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new Error(message);
  }

  return value as MutableJSONObject;
}

function toArray(value: unknown, message: string): MutableJSONObject[] {
  if (!Array.isArray(value)) {
    throw new Error(message);
  }

  return value as MutableJSONObject[];
}

function cloneJson<T extends JSONValue>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function updateWallet(componentList: MutableJSONObject[], coins: number) {
  let updated = false;

  for (const component of componentList) {
    if (component.name === "Wallet" && typeof component.data === "string") {
      try {
        const payload = JSON.parse(component.data) as MutableJSONObject;
        payload.usesCurrencySystem = true;
        payload.coins = coins;

        if (typeof payload.currency === "object" && payload.currency !== null) {
          (payload.currency as MutableJSONObject).Coins = coins;
        }

        component.data = JSON.stringify(payload);
        updated = true;
      } catch (error) {
        throw new Error("Impossible de mettre à jour le portefeuille du joueur");
      }
    }
  }

  if (!updated) {
    throw new Error("Aucun composant Wallet trouvé pour le joueur ciblé");
  }
}

function extractWalletCoins(componentList: MutableJSONObject[]): number | null {
  for (const component of componentList) {
    if (component.name === "Wallet" && typeof component.data === "string") {
      try {
        const payload = JSON.parse(component.data) as MutableJSONObject;
        if (typeof payload.currency === "object" && payload.currency !== null) {
          const coins = (payload.currency as MutableJSONObject).Coins;
          return typeof coins === "number" ? coins : null;
        }
      } catch (error) {
        return null;
      }
    }
  }

  return null;
}

function randomId() {
  if (typeof crypto !== "undefined" && "randomUUID" in crypto) {
    return crypto.randomUUID();
  }

  return `uid-${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

function findPlayerObject(objects: MutableJSONObject[], playerIndex: number) {
  const name = `Player ${playerIndex + 1}`;
  return objects.find((entry) => entry.name === name) ?? null;
}

function nextNumericValue(objects: MutableJSONObject[], key: string, fallback: number): number {
  let maxValue = fallback;

  for (const obj of objects) {
    const value = obj[key];
    if (typeof value === "number" && value > maxValue) {
      maxValue = value;
    }
  }

  return maxValue + 1;
}

export function setPlayerCoins(doc: JSONValue, options: CoinsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, coins } = options;

  const campaigns = toArray(toObject(root, "Sauvegarde invalide").campaigns, "La sauvegarde ne contient aucune campagne");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campagne introuvable");
  }

  const campaignObject = toObject(campaign, "Campagne invalide");

  if (campaignObject.carryForward) {
    const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
    const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
    if (carry[key]) {
      const currency = toObject(carry[key], "Devise du joueur invalide");
      currency.Coins = coins;
      carry.usesCurrencySystem = true;
    }

    const legacyKey = playerIndex === 0 ? "coins1" : "coins2";
    if (legacyKey in carry) {
      carry[legacyKey] = coins;
    }
  }

  const islands = campaignObject._islands ? toArray(campaignObject._islands, "Aucune île trouvée pour la campagne") : [];
  const island = islands[islandIndex];
  if (island) {
    const islandObj = toObject(island, "Données d'île invalides");
    if (Array.isArray(islandObj.monarchy?.characters)) {
      const characters = islandObj.monarchy.characters as MutableJSONObject[];
      const target = characters[playerIndex];
      if (target) {
        toObject(target, "Personnage invalide").currency = coins;
      }
    }

    if (Array.isArray(islandObj.objects)) {
      const objects = islandObj.objects as MutableJSONObject[];
      const player = findPlayerObject(objects, playerIndex);
      if (player) {
        const components = Array.isArray(player.componentData2)
          ? (player.componentData2 as MutableJSONObject[])
          : [];
        updateWallet(components, coins);
      }
    }
  }

  return root;
}

export function setPlayerGems(doc: JSONValue, options: GemsOptions): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, gems } = options;

  const campaigns = toArray(toObject(root, "Sauvegarde invalide").campaigns, "La sauvegarde ne contient aucune campagne");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campagne introuvable");
  }

  const campaignObject = toObject(campaign, "Campagne invalide");

  if (campaignObject.carryForward) {
    const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
    const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
    if (carry[key]) {
      const currency = toObject(carry[key], "Devise du joueur invalide");
      currency.Gems = gems;
      carry.usesCurrencySystem = true;
    }

    const legacyKey = playerIndex === 0 ? "gems1" : "gems2";
    if (legacyKey in carry) {
      carry[legacyKey] = gems;
    }
  }

  const islands = campaignObject._islands ? toArray(campaignObject._islands, "Aucune île trouvée pour la campagne") : [];
  const island = islands[islandIndex];
  if (island) {
    const islandObj = toObject(island, "Données d'île invalides");
    if (Array.isArray(islandObj.objects)) {
      const objects = islandObj.objects as MutableJSONObject[];
      const player = findPlayerObject(objects, playerIndex);
      if (player && Array.isArray(player.componentData2)) {
        for (const component of player.componentData2 as MutableJSONObject[]) {
          if (component.name === "Wallet" && typeof component.data === "string") {
            try {
              const payload = JSON.parse(component.data) as MutableJSONObject;
              payload.usesCurrencySystem = true;
              if (typeof payload.currency === "object" && payload.currency !== null) {
                (payload.currency as MutableJSONObject).Gems = gems;
              }
              component.data = JSON.stringify(payload);
            } catch (error) {
              console.warn("Impossible de mettre à jour les gemmes du portefeuille", error);
            }
          }
        }
      }
    }
  }

  return root;
}

export function spawnArchers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, {
    prefabs: [
      "Prefabs/Characters/Archer",
      "Prefabs/Characters/norselands/Archer_norselands"
    ],
    namePrefix: "Archer",
    positionError: "Position de l'archer invalide",
    duplicateName: "Archer"
  });
}

interface SpawnConfig {
  prefabs: readonly string[];
  namePrefix: string;
  positionError: string;
  duplicateName: string;
}

const DEFAULT_CONFIGS = {
  worker: {
    prefabs: ["Prefabs/Characters/Worker", "Prefabs/Characters/norselands/Worker_norselands"] as const,
    namePrefix: "Worker",
    positionError: "Position du travailleur invalide",
    duplicateName: "Worker",
  },
  farmer: {
    prefabs: ["Prefabs/Characters/Farmer", "Prefabs/Characters/norselands/Farmer_norselands"] as const,
    namePrefix: "Farmer",
    positionError: "Position du fermier invalide",
    duplicateName: "Farmer",
  },
  pikeman: {
    prefabs: ["Prefabs/Characters/Pikeman", "Prefabs/Characters/norselands/Knight_norselands", "Prefabs/Characters/norselands/Pikeman_norselands"] as const,
    namePrefix: "Pikeman",
    positionError: "Position du piquier invalide",
    duplicateName: "Pikeman",
  },
} satisfies Record<string, SpawnConfig>;

function findTemplateByPrefab(objects: MutableJSONObject[], prefabs: readonly string[]): MutableJSONObject | null {
  return (
    objects.find((obj) => {
      const prefab = obj.prefabPath;
      if (typeof prefab !== "string") {
        return false;
      }

      return prefabs.some((candidate) => prefab.includes(candidate));
    }) ?? null
  );
}

function spawnUnitsByConfig(
  doc: JSONValue,
  options: SpawnArchersOptions,
  config: SpawnConfig
): JSONValue {
  const root = cloneJson(doc);
  const { campaignIndex, islandIndex, playerIndex, count } = options;

  if (count <= 0) {
    throw new Error("Le nombre doit être positif");
  }

  const campaigns = toArray(toObject(root, "Sauvegarde invalide").campaigns, "La sauvegarde ne contient aucune campagne");
  const campaign = campaigns[campaignIndex];
  if (!campaign) {
    throw new Error("Campagne introuvable");
  }

  const campaignObject = toObject(campaign, "Campagne invalide");
  const islands = campaignObject._islands ? toArray(campaignObject._islands, "Aucune île trouvée pour la campagne") : [];
  const island = islands[islandIndex];
  if (!island) {
    throw new Error("Île introuvable");
  }

  const islandObj = toObject(island, "Données d'île invalides");
  const objects = toArray(islandObj.objects, "Aucun objet trouvé sur l'île");

  const template = findTemplateByPrefab(objects, config.prefabs);
  if (!template) {
    throw new Error(`Impossible de trouver un ${config.duplicateName.toLowerCase()} existant comme modèle`);
  }

  const playerObj = findPlayerObject(objects, playerIndex);
  const spawnSource = toObject(playerObj ?? template, "Objet d'inspiration invalide");
  const position = spawnSource.localPosition && typeof spawnSource.localPosition === "object"
    ? toObject(spawnSource.localPosition, config.positionError)
    : { x: 0, y: 0, z: 0 };

  let nextNetId = nextNumericValue(objects, "netID", 2000);
  let nextCreateOrder = nextNumericValue(objects, "createOrder", 20000);

  const baseWallet = Array.isArray(template.componentData2)
    ? extractWalletCoins(template.componentData2 as MutableJSONObject[])
    : null;

  for (let i = 0; i < count; i += 1) {
    const clone = cloneJson(template as JSONValue) as MutableJSONObject;
    clone.name = `${config.namePrefix} ${randomId()}`;
    clone.uniqueID = randomId();
    clone.netID = nextNetId++;
    clone.createOrder = nextCreateOrder++;

    if (clone.localPosition && typeof clone.localPosition === "object") {
      const loc = toObject(clone.localPosition, config.positionError);
      loc.x = typeof position.x === "number" ? position.x + i * 0.5 : i * 0.5;
      loc.y = typeof position.y === "number" ? position.y : 0;
      loc.z = typeof position.z === "number" ? position.z : 0;
    }

    const components = Array.isArray(clone.componentData2)
      ? (clone.componentData2 as MutableJSONObject[])
      : [];
    if (baseWallet !== null) {
      updateWallet(components, baseWallet);
    }
    objects.push(clone);
  }

  return root;
}

export function spawnWorkers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.worker);
}

export function spawnFarmers(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.farmer);
}

export function spawnPikemen(doc: JSONValue, options: SpawnArchersOptions): JSONValue {
  return spawnUnitsByConfig(doc, options, DEFAULT_CONFIGS.pikeman);
}

export function getPlayerCoins(doc: JSONValue | null, options: CoinsQueryOptions): number | null {
  if (!doc) {
    return null;
  }

  try {
    const { campaignIndex, islandIndex, playerIndex } = options;
    const root = toObject(doc, "Sauvegarde invalide");
    const campaigns = Array.isArray(root.campaigns) ? (root.campaigns as MutableJSONObject[]) : null;
    if (!campaigns) {
      return null;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign) {
      return null;
    }

    const campaignObject = toObject(campaign, "Campagne invalide");

    if (campaignObject._islands) {
      const islands = toArray(campaignObject._islands, "Données d'îles invalides");
      const island = islands[islandIndex];
      if (island) {
        const islandObj = toObject(island, "Données d'île invalides");

        if (islandObj.monarchy && Array.isArray(islandObj.monarchy.characters)) {
          const characters = islandObj.monarchy.characters as MutableJSONObject[];
          const target = characters[playerIndex];
          if (target) {
            const charObj = toObject(target, "Personnage invalide");
            if (typeof charObj.currency === "number") {
              return charObj.currency;
            }
          }
        }

        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player) {
            const components = Array.isArray(player.componentData2)
              ? (player.componentData2 as MutableJSONObject[])
              : [];
            const coins = extractWalletCoins(components);
            if (typeof coins === "number") {
              return coins;
            }
          }
        }
      }
    }

    if (campaignObject.carryForward) {
      const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
      const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
      if (carry[key]) {
        const currency = toObject(carry[key], "Devise du joueur invalide");
        const coins = currency.Coins;
        if (typeof coins === "number") {
          return coins;
        }
      }
    }

    return null;
  } catch (error) {
    console.warn("Impossible de lire le nombre de pièces", error);
    return null;
  }
}

export function getPlayerGems(doc: JSONValue | null, options: GemsQueryOptions): number | null {
  if (!doc) {
    return null;
  }

  try {
    const { campaignIndex, islandIndex, playerIndex } = options;
    const root = toObject(doc, "Sauvegarde invalide");
    const campaigns = Array.isArray(root.campaigns) ? (root.campaigns as MutableJSONObject[]) : null;
    if (!campaigns) {
      return null;
    }

    const campaign = campaigns[campaignIndex];
    if (!campaign) {
      return null;
    }

    const campaignObject = toObject(campaign, "Campagne invalide");

    if (campaignObject._islands) {
      const islands = toArray(campaignObject._islands, "Données d'îles invalides");
      const island = islands[islandIndex];
      if (island) {
        const islandObj = toObject(island, "Données d'île invalides");

        if (Array.isArray(islandObj.objects)) {
          const objects = islandObj.objects as MutableJSONObject[];
          const player = findPlayerObject(objects, playerIndex);
          if (player && Array.isArray(player.componentData2)) {
            for (const component of player.componentData2 as MutableJSONObject[]) {
              if (component.name === "Wallet" && typeof component.data === "string") {
                try {
                  const payload = JSON.parse(component.data) as MutableJSONObject;
                  if (typeof payload.currency === "object" && payload.currency !== null) {
                    const gems = (payload.currency as MutableJSONObject).Gems;
                    if (typeof gems === "number") {
                      return gems;
                    }
                  }
                } catch (error) {
                  console.warn("Impossible de lire les gemmes du portefeuille", error);
                }
              }
            }
          }
        }
      }
    }

    if (campaignObject.carryForward) {
      const carry = toObject(campaignObject.carryForward, "Données carryForward invalides");
      const key = playerIndex === 0 ? "currencyP1" : "currencyP2";
      if (carry[key]) {
        const currency = toObject(carry[key], "Devise du joueur invalide");
        const gems = currency.Gems;
        if (typeof gems === "number") {
          return gems;
        }
      }
    }

    return null;
  } catch (error) {
    console.warn("Impossible de lire le nombre de gemmes", error);
    return null;
  }
}
