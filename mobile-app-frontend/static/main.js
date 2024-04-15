// let sts = new scrollToSmooth();
console.oldLog = console.log;

function getOperationId() {
	return +document.body.dataset.operationId;
}

function getFinishAutoOperation() {
	return document.body.dataset.finishAutoOperation || null;
}

function scrollToOperation(operationId) {
	console.log('scrolling...');
	document.getElementById(`operation${operationId}`)?.scrollIntoView({
		behavior: 'smooth',
		block: 'center',
	});
	// let newEl = document.getElementById(`operation${operationId}`);
	// sts.scrollTo(`#operation${operationId}`);
	// oldEl?.classList.remove('selected');
	// newEl.classList.add('selected');
}

function setActiveOperationId(operationId) {
	document.body.dataset.operationId = operationId;
	var evt = document.createEvent('HTMLEvents');
	evt.initEvent('paste', true, false);
	document.querySelector('main').dispatchEvent(evt);
}

function setFinishAutoOperation(finishAutoOperation) {
	document.body.dataset.finishAutoOperation = finishAutoOperation;
	var evt = document.createEvent('HTMLEvents');
	evt.initEvent('paste', true, false);
	document.querySelector('main').dispatchEvent(evt);
}

function callAction(callActionData) {
	delete document.body.dataset.operationId;
	delete document.body.dataset.finishAutoOperation;
	console.log('callAction', callActionData);
	const { type, task_id, operation_id, custom_data } = callActionData;
	switch (type) {
		case 'StartOperation':
			// setActiveOperationId(operation_id); // Отключено!
			return true;
		case 'NoOperation':
		case 'RemovePermanentNotification':
			// setActiveOperationId(0); // Отключено!
			return true;
		case 'EndOperation':
		case 'EndOperationLast':
		case 'EndOperationRest':
			return true;
		case 'FinishAutoOperation':
			document.location.href = custom_data;
			// setFinishAutoOperation(JSON.stringify(callActionData));
			return true;
		case 'OpenNewTasksTab':
			document.location.href = '/new-tasks';
			return true;
	}
	return false;
}

function warningMessage(title, text) {
	new Notify({
		status: 'warning',
		title,
		text,
		effect: 'fade',
		speed: 2000,
		customClass: null,
		customIcon: null,
		showIcon: true,
		showCloseButton: true,
		autoclose: true,
		autotimeout: 1000,
		gap: 20,
		distance: 20,
		type: 1,
		position: 'center',
	});
}

function errorMessage(title, text) {
	new Notify({
		status: 'error',
		title,
		text,
		effect: 'fade',
		speed: 1300,
		customClass: null,
		customIcon: null,
		showIcon: true,
		showCloseButton: true,
		autoclose: true,
		autotimeout: 1000,
		gap: 20,
		distance: 20,
		type: 1,
		position: 'center',
	});
}

function successMessage(title, text) {
	new Notify({
		status: 'success',
		title,
		text,
		effect: 'fade',
		speed: 1000,
		customClass: null,
		customIcon: null,
		showIcon: true,
		showCloseButton: true,
		autoclose: true,
		autotimeout: 1000,
		gap: 20,
		distance: 20,
		type: 1,
		position: 'center',
	});
}

async function startBridge() {
	return new Promise((resolve) => {
		startAppHelper(() => {
			resolve(true);
		});
	});
}

function startAppHelper(f) {
	if (typeof(window.Bridge) == 'undefined') {
		return;
	}

	if (Bridge.initialized) {
		f();
	} else {
	   	Bridge.afterInitialize = f;
	}
}

const sounds = {
	short_sound: new Audio('/static/sounds/short_sound.mp3'),
	long_sound: new Audio('/static/sounds/long_sound.mp3'),
};

function enableRemoteConsole() {
	console.log = console.re.log;
}

function disableRemoteConsole() {
	console.log = console.oldLog;
}

function runVoidCommand(command, jsonStringArgument) {
	if (typeof(Bridge) != 'undefined' && Bridge.interfaces?.Android) {
		console.log('runVoidCommand on AndroidBridge');
		Bridge.interfaces.Android[command](`${jsonStringArgument}`);
	} else {
		console.log('runVoidCommand on JSInterfaceDesktop');
		window.JSInterfaceDesktop[command](`${jsonStringArgument}`);
	}
}

function getTime(it) {
	console.log('getTime', it);
	if (it.start) {
		return `${it.start.hours.toString().padStart(2, '0')}:${it.start.minutes.toString().padStart(2, '0')}:${it.start.seconds.toString().padStart(2, '0')} - ${it.end.hours.toString().padStart(2, '0')}:${it.end.minutes.toString().padStart(2, '0')}:${it.end.seconds.toString().padStart(2, '0')}`;
	} else {
		return `??:??:?? - ${it.end.hours.toString().padStart(2, '0')}:${it.end.minutes.toString().padStart(2, '0')}:${it.end.seconds.toString().padStart(2, '0')}`;
	}
}

window.JSInterfaceDesktop = {
	alarms: [],
	cancelAllAlarms: function() {
		console.log('JSInterfaceDesktop cancelAllAlarms');
		for (let cancel_alarm of this.alarms) {
			cancel_alarm();
		}
	},
	playSound: function(sound) {
		sounds[sound].play();
	},
	registerAlarms: function(str_alarms) {
		console.log(`JSInterfaceDesktop registerAlarms`);
		this.cancelAllAlarms();
		
		let alarms = JSON.parse(str_alarms);
		if (alarms.length == 0) {
			return;
		}
	
		console.log(`registering ${alarms.length} alarms - `,  alarms);

		const filtered = alarms.filter(it => ['StartOperation', 'EndOperation', 'EndOperationLast'].includes(it._type));
		const operations = [];
		if (filtered[0]._type == 'EndOperation' || filtered[0]._type == 'EndOperationLast') {
			filtered.unshift({ _type: 'StartOperation', _operation_id: filtered[0]._operation_id, time: null });
		}

		console.log(`filtered alarms - `,  filtered);

		for (let i = 0; i < filtered.length; i++) {
			if (i % 2 == 0) {
				operations.push({ id: filtered[i]._operation_id, start: filtered[i].time });
			} else {
				operations[operations.length - 1].end = filtered[i].time;
			}
		}

		console.log('operations', operations);

		console.log(`${operations.map(it => `[${it.id}] ${getTime(it)}`).join('\n')}`);

		alarms.forEach((alarm, index) => {
			const { hours, minutes, seconds } = alarm.time;
			// console.log('registering alarm', hours, minutes, seconds);
			const cur_seconds = new Date().getSeconds();
			const date = moment().set('hour', hours).set('minute', minutes).set('second', seconds).toDate();
			// console.log('moment date', date);
			// const date = moment().add(2 + 2 * index, 'seconds').toDate();
			console.log('registered date', date);
			const cancel_alarm = alarmLib(date, () => {
				console.log('alarm!', alarm);
				if (alarm.play_sound?.sound_file) {
					this.playSound(alarm.play_sound.sound_file);
				}
				if (alarm?.alarm_action?.js_action) {
					callAction(JSON.parse(alarm.alarm_action.js_action));
				}
			});
			this.alarms.push(cancel_alarm);
		});
	},
	// FIX pEUa9B
	// loadFromStorage(),,, {
	// 	// TODO!!!
	// }
};

startBridge().then(() => {
	console.log('Bridge connected!');
});