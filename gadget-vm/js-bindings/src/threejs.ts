/**
 * ThreeJS integration for Gadget VM
 */
import { createVM, parse, process } from './index.js';
import type { Scene, Object3D } from 'three';

export interface GadgetThreeJSIntegration {
  scene: Scene;
  vm: any; // GadgetVM instance
}

/**
 * Initialize Gadget VM with ThreeJS scene
 */
export async function initThreeJSIntegration(scene: Scene): Promise<GadgetThreeJSIntegration> {
  const vm = await createVM();
  
  return {
    scene,
    vm,
  };
}

/**
 * Load and parse a gadget definition, then integrate with ThreeJS scene
 */
export async function loadGadget(
  integration: GadgetThreeJSIntegration,
  source: string
): Promise<void> {
  const result = await parse(source);
  // TODO: Parse result and create/update ThreeJS objects, colliders, etc.
  console.log('Gadget loaded:', result);
}

/**
 * Execute an action in the ThreeJS context
 */
export async function executeAction(
  integration: GadgetThreeJSIntegration,
  action: string,
  target?: Object3D
): Promise<void> {
  const result = await process(action);
  // TODO: Map actions to ThreeJS operations (animations, sounds, etc.)
  console.log('Action executed:', result);
}
