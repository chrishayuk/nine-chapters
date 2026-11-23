// The counting board in three dimensions: three.js for the board and its
// rods, Rapier for the rods taken off it.
//
// A figure is set out in rods, a digit to a place, the units upright (縱) and
// the tens lying (橫), alternating up the places as Sunzi says (一縱十橫). A
// digit up to five is that many rods; six to nine is one rod across for five
// and the rest. Red rods are 正 (positive), black 負 (negative); an empty
// place is left empty. Columns run right to left, as the text sets them.
//
// Between two boards, a rod that is in both stays where it is. A rod no
// longer wanted is handed to the physics engine and knocked off the felt;
// a new one is dropped into its place.
//
// Loaded from a CDN, because this page has no build step. If WebGL or the
// CDN is missing, mount() resolves to null and the page carries on with the
// board as text.

const THREE_URL = "https://esm.sh/three@0.170.0";
const CONTROLS_URL = "https://esm.sh/three@0.170.0/examples/jsm/controls/OrbitControls.js";
const RAPIER_URL = "https://esm.sh/@dimforge/rapier3d-compat@0.14.0";

// sizes, in rod lengths
const ROD = 1.0; // length
const R = 0.045; // radius
const GAP = 0.2; // between parallel rods
const SLOT = 1.25; // one place
const ROW = 1.55; // one position, top to bottom
const COLGAP = 0.9; // between columns

const COLOURS = { pos: 0xb3261e, neg: 0x1d1a17 };

/** The rods of one figure, relative to its cell's right edge and centre line.
 *  Each: {x, z, along: "x"|"z"}; x grows leftwards from the units place. */
export function rodsOf(value) {
  const out = [];
  let n = Math.abs(value);
  let place = 0;
  while (n > 0) {
    const d = n % 10;
    const upright = place % 2 === 0; // 縱 for units, hundreds …; 橫 for tens …
    const cx = -(place + 0.5) * SLOT;
    if (d > 0) {
      const ones = d > 5 ? d - 5 : d;
      const five = d > 5;
      if (upright) {
        // ones: upright rods side by side; five: one rod lying across the top
        const width = (ones - 1) * GAP;
        for (let k = 0; k < ones; k++) out.push({ x: cx - width / 2 + k * GAP, z: five ? 0.14 : 0, along: "z", place, k });
        if (five) out.push({ x: cx, z: -ROD / 2 + 0.02, along: "x", place, k: 9 });
      } else {
        const depth = (ones - 1) * GAP;
        for (let k = 0; k < ones; k++) out.push({ x: five ? cx + 0.14 : cx, z: -depth / 2 + k * GAP, along: "x", place, k });
        if (five) out.push({ x: cx - ROD / 2 + 0.02, z: 0, along: "z", place, k: 9 });
      }
    }
    n = Math.floor(n / 10);
    place++;
  }
  return out;
}

function places(v) {
  return Math.max(1, String(Math.abs(v)).length);
}

/**
 * Mounts the board in `host`.
 * @returns {Promise<null | {show(board, opts), dispose()}>}
 */
export async function mount(host, { onError } = {}) {
  let THREE, OrbitControls, RAPIER;
  try {
    [THREE, { OrbitControls }, { default: RAPIER }] = await Promise.all([
      import(THREE_URL),
      import(CONTROLS_URL),
      import(RAPIER_URL),
    ]);
    await RAPIER.init();
  } catch (e) {
    onError?.(e);
    return null;
  }
  const canvas = document.createElement("canvas");
  let renderer;
  try {
    renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
  } catch (e) {
    onError?.(e);
    return null;
  }
  host.appendChild(canvas);
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;

  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 400);
  const controls = new OrbitControls(camera, canvas);
  controls.enableDamping = true;
  controls.maxPolarAngle = Math.PI * 0.47;

  scene.add(new THREE.HemisphereLight(0xfff4e0, 0x3a2f22, 0.9));
  const sun = new THREE.DirectionalLight(0xffffff, 1.6);
  sun.position.set(-8, 18, 10);
  sun.castShadow = true;
  sun.shadow.mapSize.set(2048, 2048);
  scene.add(sun);

  const rodGeo = new THREE.CylinderGeometry(R, R, ROD, 10);
  const mats = {
    pos: new THREE.MeshStandardMaterial({ color: COLOURS.pos, roughness: 0.55 }),
    neg: new THREE.MeshStandardMaterial({ color: COLOURS.neg, roughness: 0.45 }),
  };
  const felt = new THREE.MeshStandardMaterial({ color: 0x2f4a3a, roughness: 1 });
  const wood = new THREE.MeshStandardMaterial({ color: 0x7a5232, roughness: 0.8 });
  const floorMat = new THREE.MeshStandardMaterial({ color: 0x3b2f24, roughness: 1 });
  const glow = { actor: new THREE.MeshBasicMaterial({ color: 0xd9a45a, transparent: true, opacity: 0.28 }),
    target: new THREE.MeshBasicMaterial({ color: 0xef7a6c, transparent: true, opacity: 0.3 }) };

  // physics: the table top, the floor, and the rods knocked off
  const world = new RAPIER.World({ x: 0, y: -9.81 * 3, z: 0 });
  world.timestep = 1 / 60;
  const fallen = []; // {mesh, body, born}
  let tableCollider = null;
  const floor = world.createCollider(RAPIER.ColliderDesc.cuboid(200, 0.5, 200).setTranslation(0, -6.5, 0).setFriction(0.9));
  void floor;
  const floorMesh = new THREE.Mesh(new THREE.PlaneGeometry(400, 400), floorMat);
  floorMesh.rotation.x = -Math.PI / 2;
  floorMesh.position.y = -6;
  floorMesh.receiveShadow = true;
  scene.add(floorMesh);

  let layout = null; // the board's frame for the current page
  const frame = new THREE.Group();
  scene.add(frame);
  let rods = new Map(); // key -> {mesh, target: Vector3, from: Vector3, t}

  function label(text, size = 0.5, colour = "#efe6d4") {
    const c = document.createElement("canvas");
    const px = 128;
    const ctx = c.getContext("2d");
    ctx.font = `${px}px "Songti SC", "Noto Serif CJK SC", "Iowan Old Style", serif`;
    const w = Math.ceil(ctx.measureText(text).width) + 16;
    c.width = w;
    c.height = px * 1.3;
    ctx.font = `${px}px "Songti SC", "Noto Serif CJK SC", "Iowan Old Style", serif`;
    ctx.fillStyle = colour;
    ctx.textBaseline = "middle";
    ctx.fillText(text, 8, c.height / 2);
    const tex = new THREE.CanvasTexture(c);
    tex.colorSpace = THREE.SRGBColorSpace;
    const m = new THREE.Mesh(new THREE.PlaneGeometry((size * w) / c.height, size), new THREE.MeshBasicMaterial({ map: tex, transparent: true }));
    m.rotation.x = -Math.PI / 2;
    return m;
  }

  /** Lays out the table for boards with these columns and positions, wide
   *  enough for `widths[i]` places in column i. */
  function build(things, names, widths) {
    frame.clear();
    if (tableCollider) world.removeCollider(tableCollider, false);
    const n = names.length;
    const colW = widths.map((w) => Math.max(1, w) * SLOT);
    // x of each column's right edge; column 0 is on the right
    const right = [];
    let x = 0;
    for (let i = 0; i < n; i++) {
      right.push(x);
      x -= colW[i] + COLGAP;
    }
    const width = -x - COLGAP;
    const depth = things.length * ROW;
    const margin = 1.6;
    const cx = -width / 2;
    const top = new THREE.Mesh(new THREE.BoxGeometry(width + 2 * margin + 2.2, 0.3, depth + 2 * margin), felt);
    top.position.set(cx - 1.1, -0.15, depth / 2 - ROW / 2);
    top.receiveShadow = true;
    frame.add(top);
    const rim = new THREE.Mesh(new THREE.BoxGeometry(width + 2 * margin + 2.6, 0.5, depth + 2 * margin + 0.4), wood);
    rim.position.set(cx - 1.1, -0.45, depth / 2 - ROW / 2);
    rim.castShadow = true;
    frame.add(rim);
    tableCollider = world.createCollider(
      RAPIER.ColliderDesc.cuboid((width + 2 * margin + 2.6) / 2, 0.35, (depth + 2 * margin + 0.4) / 2)
        .setTranslation(cx - 1.1, -0.35, depth / 2 - ROW / 2)
        .setFriction(0.8)
    );
    // ruled lines between positions, and the 實 set off
    for (let p = 0; p <= things.length; p++) {
      const line = new THREE.Mesh(new THREE.PlaneGeometry(width + 0.6, p === things.length - 1 ? 0.05 : 0.02),
        new THREE.MeshBasicMaterial({ color: 0xc9b98f, transparent: true, opacity: p === things.length - 1 ? 0.5 : 0.18 }));
      line.rotation.x = -Math.PI / 2;
      line.position.set(cx, 0.005, p * ROW - ROW / 2);
      frame.add(line);
    }
    things.forEach((t, p) => {
      const l = label(t, 0.55);
      l.position.set(-width - 1.2, 0.01, p * ROW);
      frame.add(l);
    });
    const zh = names.map((nm) => ({ right: "右行", left: "左行", middle: "中行" })[nm] || `第${"〇一二三四五六七八九"[parseInt(nm)] || nm}行`);
    zh.forEach((t, i) => {
      const l = label(t, 0.5, "#d9c69b");
      l.position.set(right[i] - colW[i] / 2, 0.01, -ROW * 0.95);
      frame.add(l);
    });
    const strips = names.map((_, i) => {
      const s = new THREE.Mesh(new THREE.PlaneGeometry(colW[i] + 0.3, depth + 0.2), glow.actor);
      s.rotation.x = -Math.PI / 2;
      s.position.set(right[i] - colW[i] / 2, 0.004, depth / 2 - ROW / 2);
      s.visible = false;
      frame.add(s);
      return s;
    });
    layout = { things, names, widths, right, colW, width, depth, strips, key: things.join() + "|" + names.join() + "|" + widths.join() };
    // camera: look down the board from in front of it, a little to the reader's side
    const span = Math.max(width, depth) + 6;
    camera.position.set(cx, span * 0.9, depth + span * 0.55);
    controls.target.set(cx, 0, depth / 2 - ROW / 2);
    controls.update();
  }

  function poses(board) {
    const out = new Map();
    board.cols.forEach((col, i) => {
      if (!col) return;
      col.forEach((v, p) => {
        if (!v) return;
        const sign = v > 0 ? "pos" : "neg";
        for (const r of rodsOf(v)) {
          const key = `${i}:${p}:${r.place}:${r.k}:${r.along}:${sign}`;
          out.set(key, {
            sign,
            along: r.along,
            pos: new THREE.Vector3(layout.right[i] + r.x, R + 0.001, p * ROW + r.z),
          });
        }
      });
    });
    return out;
  }

  function rodMesh(sign, along) {
    const m = new THREE.Mesh(rodGeo, mats[sign]);
    if (along === "z") m.rotation.x = Math.PI / 2;
    else m.rotation.z = Math.PI / 2;
    m.castShadow = true;
    scene.add(m);
    return m;
  }

  function knockOff(mesh, along) {
    const p = mesh.position;
    const body = world.createRigidBody(
      RAPIER.RigidBodyDesc.dynamic().setTranslation(p.x, p.y + 0.05, p.z).setRotation(mesh.quaternion).setLinearDamping(0.1).setAngularDamping(0.4)
    );
    world.createCollider(RAPIER.ColliderDesc.capsule(ROD / 2 - R, R).setDensity(2).setFriction(0.6).setRestitution(0.2), body);
    // off the near edge of the felt, towards the reader, with a turn
    const side = Math.random() - 0.5;
    body.applyImpulse({ x: side * 0.1, y: 0.16 + Math.random() * 0.06, z: 0.55 + Math.random() * 0.2 }, true);
    body.applyTorqueImpulse({ x: (Math.random() - 0.5) * 0.01, y: (Math.random() - 0.5) * 0.02, z: (Math.random() - 0.5) * 0.01 }, true);
    void along;
    fallen.push({ mesh, body, born: performance.now() });
    // keep the floor from filling up
    while (fallen.length > 600) {
      const f = fallen.shift();
      scene.remove(f.mesh);
      world.removeRigidBody(f.body);
    }
  }

  /**
   * Shows a board. `widths`: places per column for the whole page (so the
   * layout holds still); `actor`, `target`: columns to light; `instant`: no
   * animation (a jump).
   */
  function show(board, { widths, actor = null, target = null, instant = false } = {}) {
    const key = board.things.join() + "|" + board.names.join() + "|" + widths.join();
    if (!layout || layout.key !== key) {
      for (const r of rods.values()) scene.remove(r.mesh);
      rods = new Map();
      build(board.things, board.names, widths);
    }
    layout.strips.forEach((s, i) => {
      s.visible = i === actor || i === target;
      s.material = i === target ? glow.target : glow.actor;
    });
    const want = poses(board);
    const next = new Map();
    for (const [k, r] of rods) {
      if (want.has(k)) {
        const w = want.get(k);
        if (!w.pos.equals(r.target)) {
          r.from = r.mesh.position.clone();
          r.target = w.pos;
          r.t = instant ? 1 : 0;
          if (instant) r.mesh.position.copy(w.pos);
        }
        next.set(k, r);
      } else if (instant) {
        scene.remove(r.mesh);
      } else {
        r.mesh.position.copy(r.target);
        knockOff(r.mesh, r.along);
      }
    }
    let delay = 0;
    for (const [k, w] of want) {
      if (next.has(k)) continue;
      const mesh = rodMesh(w.sign, w.along);
      const from = w.pos.clone().add(new THREE.Vector3(0, 3.5, -0.6));
      mesh.position.copy(instant ? w.pos : from);
      next.set(k, { mesh, along: w.along, target: w.pos, from, t: instant ? 1 : -delay });
      delay += 0.004;
    }
    rods = next;
  }

  function resize() {
    const w = host.clientWidth || 600;
    const h = host.clientHeight || 420;
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  }
  const ro = new ResizeObserver(resize);
  ro.observe(host);
  resize();

  let last = performance.now();
  let running = true;
  function frameLoop(now) {
    if (!running) return;
    const dt = Math.min(0.05, (now - last) / 1000);
    last = now;
    world.step();
    for (const f of fallen) {
      const t = f.body.translation();
      const q = f.body.rotation();
      f.mesh.position.set(t.x, t.y, t.z);
      f.mesh.quaternion.set(q.x, q.y, q.z, q.w);
    }
    for (const r of rods.values()) {
      if (r.t >= 1) continue;
      r.t += dt * 3.2;
      const t = Math.max(0, Math.min(1, r.t));
      const e = 1 - Math.pow(1 - t, 3);
      r.mesh.position.lerpVectors(r.from, r.target, e);
    }
    controls.update();
    renderer.render(scene, camera);
    requestAnimationFrame(frameLoop);
  }
  requestAnimationFrame(frameLoop);

  return {
    show,
    rodCount: () => rods.size,
    clearFloor() {
      for (const f of fallen) {
        scene.remove(f.mesh);
        world.removeRigidBody(f.body);
      }
      fallen.length = 0;
    },
    dispose() {
      running = false;
      ro.disconnect();
      renderer.dispose();
      canvas.remove();
    },
  };
}

/** Places per column over every board of a run, so the layout holds still. */
export function widthsOf(boards) {
  const w = [];
  for (const b of boards) {
    b.cols.forEach((c, i) => {
      if (!c) return;
      w[i] = Math.max(w[i] || 1, ...c.map(places));
    });
  }
  const n = Math.max(0, ...boards.map((b) => b.cols.length));
  for (let i = 0; i < n; i++) w[i] ??= 1;
  return w;
}
