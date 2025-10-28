import Worker from "./worker.js?worker"

export class WorkerPool {
    constructor(size) {
        this.size = size;
        this.workers = [];
        this.tasks = [];
        this._initializeWorkers();
    }

    // 初始化 Workers
    _initializeWorkers() {
        for (let i = 0; i < this.size; i++) {
            const worker = new Worker();
            worker.onmessage = this._handleWorkerMessage.bind(this);
            worker.onerror = this._handleWorkerError.bind(this);
            this.workers.push(worker);
        }
    }

    // 添加任务到队列
    addTask(taskData) {
        const task = new Promise((resolve, reject) => {
            this.tasks.push({ taskData, resolve, reject });
        });
        this._assignTask();
        return task;
    }

    // 将任务分配给空闲的 Worker
    _assignTask() {
        const availableWorker = this.workers.find(worker => !worker.busy);
        if (availableWorker && this.tasks.length > 0) {
            const { taskData, resolve, reject } = this.tasks.shift();
            availableWorker.busy = true;
            availableWorker.postMessage(taskData);
            availableWorker.onmessage = (e) => {
                resolve(e.data);
                availableWorker.busy = false;
                this._assignTask();
            };
            availableWorker.onerror = reject;
        }
    }

    // 处理 Worker 返回的数据
    _handleWorkerMessage(event) {
        // 处理消息
        console.log(event)
    }

    // 错误处理
    _handleWorkerError(error) {
        console.error("Worker error: ", error);
    }
}

