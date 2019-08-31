'use strict';
var THREE = require('three');

var pieceDisplaySize = 400;
var blockDisplaySize = 20;
var piecesDisplay = {
	div: document.getElementById('piecesDiv')
}
var rotationsDisplay = {
	div: document.getElementById('rotationsDiv')
}
var solutionDisplay = {
	div: document.getElementById('solutionDiv')
}



var module;
var pieces;
function positionToCoord(position) {
	return { x: Math.floor(position % 5), y: Math.floor((position % 25) / 5), z: Math.floor(position / 25), position: position }
}
function coordToPosition(coord) {
	return coord.x + coord.y * 5 + coord.z * 25
}

function bitsToCoord(bits) {
	var blocks = []
	var byteSize = 0x10000
	for (var b = 0; b < 4; b++) {
		var byte = Math.floor(bits / Math.pow(byteSize, b)) % byteSize;
		for (var i = 0; i < 16; i++) {
			if (((byte >> i) & 1) == 1) {
				var coord = positionToCoord(i + b * 16)
				blocks.push(coord)

			}
		}
	}
	return blocks
}

init(piecesDisplay);
init(rotationsDisplay);
init(solutionDisplay);

function pause(millis) {
	return new Promise(function (resolve, reject) {
		setTimeout(resolve, millis);
	})
}

begin()

var allPieces
var solution

var dragging = false

async function begin() {
	var blocksModule = await



		import('./blocks.js');
	pieces = blocksModule.pieces;
	pieces.forEach((piece, p) => {
		piece.position = 0;
		piece.rotations = [];
		piece.blocks.forEach(block => {
			block.fill_color = 0
			block.edge_color = 0
		})
		piece.bits = 0;
		// 		piece.bots={1:0,2:0};
		piece.used = false;
		piece.number = p;
	})

	const rust = import("../pkg/index.js");
	module = await rust;


	renderPieces(pieces, piecesDisplay)
	beginAnimation(pieces, piecesDisplay)
	// console.log(module.solve())

	// 			var newPieces = module.get_pieces_js(pieces);			
	// 			// 									console.log(newPieces.map(p=>p.rotations.length))
	// 			newPieces.forEach(piece => {				
	// 				var blocks=bitsToCoord(piece.bits)				
	// 				blocks.forEach(block=>{
	// 					var found=piece.blocks.find(b=>b.x==block.x && b.y==block.y && b.z==block.z)
	// 					if (!found){
	// 						piece.blocks.forEach(b=>b.position=coordToPosition(b))
	// 					console.log(blocks,piece.blocks)
	// 					console.log(blocks.map(b=>coordToPosition(b)))
	// 					console.log(piece.blocks.map(b=>coordToPosition(b)))					
	// 					}
	// 				})

	// 			})

	// 			var allPieces = [];
	// 			for (var i = 0; i < 2; i++) {
	// 				console.log(i);
	// 				allPieces.push(pieces.map(piece => module.get_legal_rotations_js(piece, i)));
	// 			}
	// console.time('allPieces')
	allPieces = module.get_legal_rotations_all_js(pieces);
	// 			console.timeEnd('allPieces')
	// 				console.log(allPieces);




	// 			console.log(allPieces.reduce((p,c)=>p+c.reduce((p2,c2)=>p2+c2.rotations.length,0)   ,0))
	var dragStart = {};
	var dragMove = {};
	var dragPosition={x:0,y:0};

	document.getElementById('solutionDiv').addEventListener('mousedown', function (e) {
		dragStart.x = e.clientX;
		dragStart.y = e.clientY;
		dragging = true;
		animateDrag() ;
	})
	document.getElementById('solutionDiv').addEventListener('mousemove', function (e) {
		if (dragging){
		dragMove.x = dragStart.x - e.clientX;
		dragMove.y = dragStart.y - e.clientY;
		statusOutput.innerText = dragMove.x + ', ' + dragMove.y + ', ' + dragPosition.x;
		}		
	})
		function animateDrag() {
			if (dragging){
				requestAnimationFrame(animateDrag);
				solutionDisplay.scene.children.forEach(child => {
					child.rotation.x = -(dragPosition.y+dragMove.y )/1000;
					child.rotation.y = -(dragPosition.x+dragMove.x)/1000;
				})
				solutionDisplay.renderer.render(solutionDisplay.scene, solutionDisplay.camera);
			}
			};
	document.addEventListener('mouseup', function (e) {	
	dragPosition.x+= dragMove.x;
		dragPosition.y += dragMove.y ;
		statusOutput.innerText = dragMove.x + ', ' + dragMove.y + ', ' + dragPosition.x;
			dragging = false;
	})
	var t = 0;
	document.getElementById('test').addEventListener('click', function () {
		drawSolution(allPieces, solution.slice(t, t + 1))
		t++
		if (t >= solution.length) {
			t = 0
		}
		// 	console.log(bitsToCoord(document.getElementById('position').value))
	}, false);

	
	drawSolution(allPieces, [])

	beginAnimationSolution(solutionDisplay);
	console.time('solving')
	for (var i = 0; i < 100; i++) {
		var result  = module.solve_step_js(100000,false,false);
		drawSolution(allPieces, result.solution)
		if (result.solved){		
		console.log('solution found',i)
			break;
		}
		//First solution:
		// 103 seconds
		// 		allPieces[0].forEach(p => p.used = false)
		// 		solution.forEach(sol =>			allPieces[0][sol[1]].used = true		)

		// 		console.log(solution)
		await pause(1)
	}
	console.timeEnd('solving')
	// 			renderSolution(solution, solutionDisplay)
	// 			console.log(solution)



	// 			var rotations = module.get_rotations(pieces[0]);

	// 			console.log(pieces[0], rotations[0]);

	// 			renderPieces(rotations, rotationsDisplay)
	// 			beginAnimation(rotations, rotationsDisplay)


	// 			beginAnimation(sol, solutionDisplay)



	// 
}

window.draw_solution_js = function (solution) {
	reRenderSolution(solution, solutionDisplay);
}

function drawSolution(allPieces, solution) {
	solution = solution.map(sol => allPieces[sol[0]][sol[1]].rotations[sol[2]])
	// 	console.log(solution)
	reRenderSolution(solution, solutionDisplay);
}


async function reRenderSolution(solution, display) {
	display.scene.children.slice().forEach(child => display.scene.remove(child))
	// 	await pause(50)
	renderSolution(solution, display)
}


rerender.addEventListener('click', function () {
	reRender(pieces, piecesDisplay)
}, false);

function reRender(pieces, display, showBoundingBox, position) {
	display.scene.children.slice().forEach(child => display.scene.remove(child))
	renderPieces(pieces, display, showBoundingBox, position)
}
var solGroup = new THREE.Group();

function renderSolution(pieces, display) {
	solGroup.children.slice().forEach(child => solGroup.remove(child))

	var blockDisplaySize = 60;
	var geometry = new THREE.BoxBufferGeometry(blockDisplaySize, blockDisplaySize, blockDisplaySize);
	var edges = new THREE.EdgesGeometry(geometry);
	var geometryBB = new THREE.BoxBufferGeometry(blockDisplaySize * 5, blockDisplaySize * 5, blockDisplaySize * 5);
	var edgesBB = new THREE.EdgesGeometry(geometryBB);
	pieces.forEach(piece => {
		piece.blocks.forEach(block => {
			var material = new THREE.MeshBasicMaterial({ color: block.fill_color ? block.fill_color : piece.fill_color });
			var cube = new THREE.Mesh(geometry, material);
			cube.position.set(block.x * blockDisplaySize, block.y * blockDisplaySize, block.z * blockDisplaySize);
			solGroup.add(cube);
			var line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: piece.edge_color }));
			line.position.set(block.x * blockDisplaySize, block.y * blockDisplaySize, block.z * blockDisplaySize);
			solGroup.add(line);
		})
	})
// 	var line = new THREE.LineSegments(edgesBB, new THREE.LineBasicMaterial({ color: 0 }));
// 	line.position.set(blockDisplaySize * 2, blockDisplaySize * 2, blockDisplaySize * 2);
// 	solGroup.add(line);
	solGroup.position.set(-blockDisplaySize , -blockDisplaySize , -blockDisplaySize );
	display.scene.add(solGroup);	
	display.renderer.render(display.scene, display.camera);
}


function renderPieces(pieces, display, showBoundingBox, position) {
	var geometry = new THREE.BoxBufferGeometry(blockDisplaySize, blockDisplaySize, blockDisplaySize);
	var edges = new THREE.EdgesGeometry(geometry);
	var geometryBB = new THREE.BoxBufferGeometry(blockDisplaySize * 5, blockDisplaySize * 5, blockDisplaySize * 5);
	var edgesBB = new THREE.EdgesGeometry(geometryBB);
	pieces.forEach((piece, p) => {
		piece.group = new THREE.Group();

		piece.blocks.forEach((block, blocks) => {
			var material = new THREE.MeshBasicMaterial({ color: block.fill_color ? block.fill_color : piece.fill_color });
			var cube = new THREE.Mesh(geometry, material);
			cube.position.set(block.x * blockDisplaySize, block.y * blockDisplaySize, block.z * blockDisplaySize);
			piece.group.add(cube);
			var line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: piece.edge_color }));
			line.position.set(block.x * blockDisplaySize, block.y * blockDisplaySize, block.z * blockDisplaySize);
			piece.group.add(line);

		})
		if (showBoundingBox) {
			var line = new THREE.LineSegments(edgesBB, new THREE.LineBasicMaterial({ color: piece.edge_color }));
			line.position.set(blockDisplaySize * 2, blockDisplaySize * 2, blockDisplaySize * 2);
			piece.group.add(line);
		}


		if (position) {
			var fill_color = 0xEEEEEE;
			var edge_color = 0;
			var material = new THREE.MeshBasicMaterial({ color: fill_color });


			for (var pos = 0; pos < position; pos++) {
				var cube = new THREE.Mesh(geometry, material);
				cube.position.set(Math.floor(pos % 5) * blockDisplaySize, Math.floor((pos % 25) / 5) * blockDisplaySize, Math.floor(pos / 25) * blockDisplaySize);
				piece.group.add(cube);
				var line = new THREE.LineSegments(edges, new THREE.LineBasicMaterial({ color: edge_color }));
				line.position.set(Math.floor(pos % 5) * blockDisplaySize, Math.floor((pos % 25) / 5) * blockDisplaySize, Math.floor(pos / 25) * blockDisplaySize);
				piece.group.add(line);

			}
		}

		let spacing = 7

		piece.group.position.set(5 * (blockDisplaySize + spacing) * (p % 5 - 2), 5 * (blockDisplaySize + spacing) * Math.floor(p / 5 - 2), 0);
		display.scene.add(piece.group);
	})

}

function init(display) {
	var div = display.div;
	var cameraFrustumAngle = 70;
	var divWidth = div.offsetWidth
	var divHeight = div.offsetHeight

	display.camera = new THREE.PerspectiveCamera(cameraFrustumAngle, divWidth / divHeight, 1, 1000);
	display.camera.position.z = 500;
	display.scene = new THREE.Scene();


	display.renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
	display.renderer.setClearColor(0xffffff, 0);
	display.renderer.setPixelRatio(window.devicePixelRatio);
	display.renderer.setSize(divWidth, divHeight);
	div.appendChild(display.renderer.domElement);
	display.camera.aspect = 1;
	display.camera.updateProjectionMatrix();
	display.renderer.render(display.scene, display.camera);

	div.addEventListener('mousedown', onMouseDown, false);
// 	div.addEventListener('mousemove', onMouseMove, false);
// 	function onMouseMove(e) {
// 		statusOutput.innerText = (e.clientX - div.offsetLeft) + ', ' + (e.clientY - div.offsetTop) + ', ' + divWidth + ', ' + divHeight
// 	}
	function onMouseDown(e) {
		if (div.id == 'piecesDiv') {
			var vectorMouse = new THREE.Vector3( //vector from camera to mouse
				-(divWidth / 2 - (e.clientX - div.offsetLeft)) * 2 / divWidth,
				(divHeight / 2 - (e.clientY - div.offsetTop)) * 2 / divHeight,
				-1 / Math.tan((cameraFrustumAngle / 2) * Math.PI / 180)); //22.5 is half of camera frustum angle 45 degree
			vectorMouse.applyQuaternion(display.camera.quaternion);
			vectorMouse.normalize();
			pieces.forEach((piece, p) => {
				var blockDistances = piece.group.children.filter(child => child.type == 'Mesh').map((block, b) => {
					let handVec = new THREE.Vector3()
					block.getWorldPosition(handVec)
					var vectorObject = new THREE.Vector3(); //vector from camera to object
					vectorObject.set(handVec.x - display.camera.position.x,
						handVec.y - display.camera.position.y,
						handVec.z - display.camera.position.z);
					vectorObject.normalize();
					return Math.floor(vectorMouse.angleTo(vectorObject) * 180 / Math.PI)

				})
				if (Math.min(...blockDistances) <= 2) {
					selectedPiece = pieces[p];
					showRotations(piece);
				}
			})
		}
	}
}

function showRotations(piece) {
	var position = parseInt(document.getElementById('position').value)

	var rotations = module.get_legal_rotations_js(piece, position);

	console.log(rotations.length)
	reRender(rotations, rotationsDisplay, true, position);
	beginAnimation(rotations, rotationsDisplay)
}
var selectedPiece;

function beginAnimation(pieces, display) {
	function animate() {
		requestAnimationFrame(animate);
		pieces.forEach(piece => {

			piece.group.rotation.x += 0.005;
			piece.group.rotation.y += 0.01;

		})
		display.renderer.render(display.scene, display.camera);
	};
	animate();
}

function beginAnimationSolution(display) {
	function animate() {
		if (!dragging){
		requestAnimationFrame(animate);
		display.scene.children.forEach(child => {
			child.rotation.x += 0.0005;
			child.rotation.y += 0.001;
		})
		


		display.renderer.render(display.scene, display.camera);
		}
	};
	animate();
}