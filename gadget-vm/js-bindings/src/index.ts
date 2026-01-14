import init, { GadgetVM } from '../wasm/gadget_vm.js';

let wasmInitialized = false;
let nextSoundId = 0;
const activeAudioElements = new Map<number, HTMLAudioElement>();

// Set up global callbacks for WASM to use
// These must be set up before WASM initialization
if (typeof window !== 'undefined') {
  /**
   * Play a sound file and return its ID
   * This is called from WASM
   */
  (window as any).playSound = (path: string): number => {
    try {
      const audio = new Audio(path);
      const soundId = nextSoundId++;
      
      audio.addEventListener('ended', () => {
        activeAudioElements.delete(soundId);
      });
      
      audio.addEventListener('error', () => {
        activeAudioElements.delete(soundId);
      });
      
      audio.play().catch(err => {
        console.error('Error playing sound:', err);
        activeAudioElements.delete(soundId);
      });
      
      activeAudioElements.set(soundId, audio);
      return soundId;
    } catch (error) {
      console.error('Error creating audio:', error);
      return -1;
    }
  };

  /**
   * Stop a sound by ID
   * This is called from WASM
   */
  (window as any).stopSound = (id: number): void => {
    const audio = activeAudioElements.get(id);
    if (audio) {
      audio.pause();
      audio.currentTime = 0;
      activeAudioElements.delete(id);
    }
  };
}

/**
 * Initialize the WASM module
 * Must be called before using the VM
 */
export async function initVM(): Promise<void> {
  if (!wasmInitialized) {
    await init();
    wasmInitialized = true;
  }
}

/**
 * Create a new Gadget VM instance
 */
export async function createVM(): Promise<GadgetVM> {
  await initVM();
  return new GadgetVM();
}

/**
 * Parse Gadget Language source code
 */
export async function parse(source: string): Promise<string> {
  const vm = await createVM();
  return vm.parse(source);
}

/**
 * Process input through the VM
 */
export async function process(input: string): Promise<string> {
  const vm = await createVM();
  return vm.process(input);
}

// Re-export for direct access if needed
export { GadgetVM };
