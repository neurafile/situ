/**
 * BabylonJS integration for Gadget VM
 */
import { createVM, parse, process } from './index.js';
import type { Scene, Mesh, AbstractMesh } from '@babylonjs/core';

export interface GadgetBabylonIntegration {
  scene: Scene;
  vm: any; // GadgetVM instance
}

/**
 * Initialize Gadget VM with BabylonJS scene
 */
export async function initBabylonIntegration(scene: Scene): Promise<GadgetBabylonIntegration> {
  const vm = await createVM();
  
  return {
    scene,
    vm,
  };
}

/**
 * Load and parse a gadget definition, then integrate with BabylonJS scene
 */
export async function loadGadget(
  integration: GadgetBabylonIntegration,
  source: string
): Promise<void> {
  const result = await parse(source);
  // TODO: Parse result and create/update BabylonJS meshes, colliders, etc.
  console.log('Gadget loaded:', result);
}

/**
 * Execute an action in the BabylonJS context
 */
export async function executeAction(
  integration: GadgetBabylonIntegration,
  action: string,
  target?: AbstractMesh
): Promise<void> {
  const result = await process(action);
  // TODO: Map actions to BabylonJS operations (animations, sounds, etc.)
  console.log('Action executed:', result);
}
