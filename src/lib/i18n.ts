export type Language = 'en' | 'fr';

export interface Translations {
  welcome: {
    title: string;
    subtitle: string;
    osDetected: string;
    defaultPath: string;
    pathHint: string;
    openFile: string;
    loading: string;
    features: {
      resources: { title: string; desc: string };
      navigation: { title: string; desc: string };
      combat: { title: string; desc: string };
      construction: { title: string; desc: string };
      recruitment: { title: string; desc: string };
    };
  };
  header: {
    title: string;
    changeFile: string;
    save: string;
    backupCreated: string;
    filePath: string;
  };
  context: {
    title: string;
    description: string;
    campaign: string;
    island: string;
  };
  tabs: {
    resources: string;
    navigation: string;
    combat: string;
    construction: string;
    recruitment: string;
  };
  resources: {
    title: string;
    playerCoins: string;
    playerGems: string;
    player: string;
    coins: string;
    gems: string;
    apply: string;
    current: string;
  };
  navigation: {
    title: string;
    fastTravel: string;
    takeOver: string;
    destroyPortals: string;
    exterminate: string;
    markTrees: string;
    updateDestination: string;
    cleanIsland: string;
    destroy: string;
    exterminateEnemies: string;
    mark: string;
    pikemen: string;
    farmers: string;
    boats: string;
    playerCoins: string;
    archers: string;
    workers: string;
    formationOffset: string;
  };
  combat: {
    title: string;
    battleFormation: string;
    deploy: string;
    positionX: string;
  };
  construction: {
    title: string;
    pimpIsland: string;
    upgrade: string;
    unitsToRecruit: string;
  };
  recruitment: {
    title: string;
    rallyPoint: string;
    archers: string;
    workers: string;
    farmers: string;
    pikemen: string;
    quantity: string;
    add: string;
    combinedRecruitment: string;
    addUnits: string;
    current: string;
  };
  status: {
    coinsUpdated: string;
    gemsUpdated: string;
    archersAdded: string;
    workersAdded: string;
    farmersAdded: string;
    pikemenAdded: string;
    destinationUpdated: string;
    islandSecured: string;
    portalsDestroyed: string;
    enemiesRemoved: string;
    formationDeployed: string;
    islandUpgraded: string;
    treesMarked: string;
    unitsAdded: string;
    noFileLoaded: string;
    saveApplied: string;
    addAtLeastOneUnit: string;
  };
}

const translations: Record<Language, Translations> = {
  en: {
    welcome: {
      title: 'Kingdom Two Crowns Save Editor',
      subtitle: 'Edit your save files easily',
      osDetected: 'System detected:',
      defaultPath: 'Default save location:',
      pathHint: 'The file picker will automatically open this folder',
      openFile: 'Open global-v35',
      loading: 'Loading...',
      features: {
        resources: { title: 'Resources', desc: 'Edit your coins and gems' },
        navigation: { title: 'Navigation', desc: 'Travel instantly between islands' },
        combat: { title: 'Combat', desc: 'Deploy your battle formations' },
        construction: { title: 'Construction', desc: 'Upgrade your kingdom' },
        recruitment: { title: 'Recruitment', desc: 'Add archers, workers and more' },
      },
    },
    header: {
      title: 'Kingdom Two Crowns Save Editor',
      changeFile: 'Change file',
      save: '💾 Save',
      backupCreated: 'Backup copy:',
      filePath: '📁',
    },
    context: {
      title: 'Context',
      description: 'Select the campaign and island to edit.',
      campaign: 'Campaign',
      island: 'Island',
    },
    tabs: {
      resources: 'Resources',
      navigation: 'Navigation',
      combat: 'Combat',
      construction: 'Construction',
      recruitment: 'Recruitment',
    },
    resources: {
      title: 'Resources',
      playerCoins: 'Player gold',
      playerGems: 'Player gems',
      player: 'Player',
      coins: 'Coins',
      gems: 'Gems',
      apply: 'Apply',
      current: 'Current:',
    },
    navigation: {
      title: 'Navigation & conquest',
      fastTravel: 'Fast travel',
      takeOver: 'Take over',
      destroyPortals: 'Destroy portals',
      exterminate: 'Exterminate enemies',
      markTrees: 'Mark trees',
      updateDestination: 'Update destination',
      cleanIsland: 'Clean island',
      destroy: 'Destroy portals',
      exterminateEnemies: 'Exterminate enemies',
      mark: 'Mark trees',
      pikemen: 'Pikemen',
      farmers: 'Farmers',
      boats: 'Boats',
      playerCoins: 'Player coins',
      archers: 'Archers',
      workers: 'Workers',
      formationOffset: 'Formation offset',
    },
    combat: {
      title: 'Combat',
      battleFormation: 'Battle formation',
      deploy: 'Deploy',
      positionX: 'Position X',
    },
    construction: {
      title: 'Construction',
      pimpIsland: 'Pimp my island',
      upgrade: 'Upgrade',
      unitsToRecruit: 'Units to recruit',
    },
    recruitment: {
      title: 'Recruitment',
      rallyPoint: 'Rally point',
      archers: 'Archers',
      workers: 'Workers',
      farmers: 'Farmers',
      pikemen: 'Pikemen',
      quantity: 'Quantity',
      add: 'Add',
      combinedRecruitment: 'Combined recruitment',
      addUnits: 'Add units',
      current: 'Current:',
    },
    status: {
      coinsUpdated: 'Coins updated',
      gemsUpdated: 'Gems updated',
      archersAdded: 'Archers added',
      workersAdded: 'Workers added',
      farmersAdded: 'Farmers added',
      pikemenAdded: 'Pikemen added',
      destinationUpdated: 'Destination updated',
      islandSecured: 'Island secured',
      portalsDestroyed: 'Portals destroyed',
      enemiesRemoved: 'Enemies removed',
      formationDeployed: 'Formation deployed',
      islandUpgraded: 'Island upgraded',
      treesMarked: 'Trees marked',
      unitsAdded: 'Units added',
      noFileLoaded: 'No file loaded',
      saveApplied: 'Save applied',
      addAtLeastOneUnit: 'Enter at least one unit to add',
    },
  },
  fr: {
    welcome: {
      title: 'Éditeur Kingdom Two Crowns',
      subtitle: 'Modifiez vos sauvegardes facilement',
      osDetected: 'Système détecté :',
      defaultPath: 'Emplacement de sauvegarde par défaut :',
      pathHint: 'Le sélecteur de fichier ouvrira automatiquement ce dossier',
      openFile: 'Ouvrir global-v35',
      loading: 'Chargement...',
      features: {
        resources: { title: 'Ressources', desc: 'Modifiez vos pièces et gemmes' },
        navigation: { title: 'Navigation', desc: 'Voyagez instantanément entre les îles' },
        combat: { title: 'Combat', desc: 'Déployez vos formations de combat' },
        construction: { title: 'Construction', desc: 'Améliorez votre royaume' },
        recruitment: { title: 'Recrutement', desc: 'Ajoutez archers, ouvriers et plus' },
      },
    },
    header: {
      title: 'Éditeur Kingdom Two Crowns',
      changeFile: 'Changer de fichier',
      save: '💾 Sauvegarder',
      backupCreated: 'Copie de sauvegarde:',
      filePath: '📁',
    },
    context: {
      title: 'Contexte',
      description: 'Sélectionnez la campagne et l\'île à modifier.',
      campaign: 'Campagne',
      island: 'Île',
    },
    tabs: {
      resources: 'Ressources',
      navigation: 'Navigation',
      combat: 'Combat',
      construction: 'Construction',
      recruitment: 'Recrutement',
    },
    resources: {
      title: 'Ressources',
      playerCoins: 'Or du joueur',
      playerGems: 'Gemmes du joueur',
      player: 'Joueur',
      coins: 'Pièces',
      gems: 'Gemmes',
      apply: 'Appliquer',
      current: 'Actuel:',
    },
    navigation: {
      title: 'Navigation & conquête',
      fastTravel: 'Voyage rapide',
      takeOver: 'Prendre le contrôle',
      destroyPortals: 'Détruire les portails',
      exterminate: 'Exterminer les ennemis',
      markTrees: 'Élaguer la forêt',
      updateDestination: 'Mettre à jour la destination',
      cleanIsland: 'Nettoyer l\'île',
      destroy: 'Détruire les portails',
      exterminateEnemies: 'Exterminer les ennemis',
      mark: 'Marquer les arbres',
      pikemen: 'Piquiers',
      farmers: 'Fermiers',
      boats: 'Bateaux',
      playerCoins: 'Pièces du joueur',
      archers: 'Archers',
      workers: 'Ouvriers',
      formationOffset: 'Décalage formation',
    },
    combat: {
      title: 'Combat',
      battleFormation: 'Formation de bataille',
      deploy: 'Déployer',
      positionX: 'Position X',
    },
    construction: {
      title: 'Construction',
      pimpIsland: 'Pimper l\'île',
      upgrade: 'Mettre à niveau',
      unitsToRecruit: 'Unités à recruter',
    },
    recruitment: {
      title: 'Recrutement',
      rallyPoint: 'Point de rassemblement',
      archers: 'Archers',
      workers: 'Ouvriers',
      farmers: 'Fermiers',
      pikemen: 'Piquiers',
      quantity: 'Quantité',
      add: 'Ajouter',
      combinedRecruitment: 'Recrutement combiné',
      addUnits: 'Ajouter les unités',
      current: 'Actuel:',
    },
    status: {
      coinsUpdated: 'Pièces mises à jour',
      gemsUpdated: 'Gemmes mises à jour',
      archersAdded: 'Archers ajoutés',
      workersAdded: 'Ouvriers ajoutés',
      farmersAdded: 'Fermiers ajoutés',
      pikemenAdded: 'Piquiers ajoutés',
      destinationUpdated: 'Destination mise à jour',
      islandSecured: 'Île sécurisée',
      portalsDestroyed: 'Portails détruits',
      enemiesRemoved: 'Ennemis supprimés',
      formationDeployed: 'Formation déployée',
      islandUpgraded: 'Île améliorée',
      treesMarked: 'Arbres marqués',
      unitsAdded: 'Unités ajoutées',
      noFileLoaded: 'Aucun fichier chargé',
      saveApplied: 'Sauvegarde appliquée',
      addAtLeastOneUnit: 'Saisissez au moins une unité à ajouter',
    },
  },
};

export function detectLanguage(): Language {
  const browserLang = navigator.language.toLowerCase();
  
  if (browserLang.startsWith('fr')) {
    return 'fr';
  }
  
  return 'en';
}

export function getTranslations(lang: Language): Translations {
  return translations[lang];
}
