// 场景初始化
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// 创建 FBX 加载器
const loader = new THREE.FBXLoader();

// 加载 FBX 文件
loader.load(
    // FBX 文件路径
    'models/character.fbx',
    // 加载成功回调
    function (object) {
        // 将模型添加到场景中
        scene.add(object);
        
        // 调整模型位置
        object.position.set(0, 0, 0);
        
        // 可选：设置动画混合器（如果模型包含动画）
        if (object.animations && object.animations.length > 0) {
            const mixer = new THREE.AnimationMixer(object);
            const action = mixer.clipAction(object.animations[0]);
            action.play();
            
            // 在动画循环中更新混合器
            const clock = new THREE.Clock();
            function animate() {
                requestAnimationFrame(animate);
                
                // 更新动画
                const delta = clock.getDelta();
                if (mixer) mixer.update(delta);
                
                renderer.render(scene, camera);
            }
            animate();
        } else {
            // 无动画的简单渲染
            function animate() {
                requestAnimationFrame(animate);
                renderer.render(scene, camera);
            }
            animate();
        }
    },
    // 加载进度回调
    function (xhr) {
        console.log((xhr.loaded / xhr.total * 100) + '% loaded');
    },
    // 加载错误回调
    function (error) {
        console.error('Error loading FBX model:', error);
    }
);

// 设置相机位置
camera.position.z = 5;