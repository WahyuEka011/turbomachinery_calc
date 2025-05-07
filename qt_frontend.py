import os
import sys
import ctypes
from ctypes import CDLL
from PyQt5.QtWidgets import (
    QApplication, QMainWindow, QVBoxLayout, QWidget, 
    QPushButton, QLabel, QTextEdit, QLineEdit
)
from PyQt5.QtCore import QThread, pyqtSignal
import logging

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class RustLibWrapper:
    def __init__(self):
        # Load the Rust library
        lib_path = os.path.expanduser("~/Documents/turbomachinery_qtpyth/turbomachinery_qtpyth/target/release/libturbomachinery_qtpyth.so")
        try:
            self.rust_lib = CDLL(lib_path)
            logger.info(f"Successfully loaded Rust library from: {lib_path}")
            
            # Set up function prototypes
            self.rust_lib.predict.restype = ctypes.POINTER(ctypes.c_float)
            self.rust_lib.predict.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.c_size_t]
            
            self.rust_lib.predict_with_rul.restype = ctypes.c_char_p
            self.rust_lib.predict_with_rul.argtypes = [ctypes.POINTER(ctypes.c_float), ctypes.c_size_t]
            
            self.rust_lib.set_qt_callback.restype = None
            self.rust_lib.set_qt_callback.argtypes = [ctypes.py_object]
            
        except Exception as e:
            logger.error(f"Failed to load Rust library: {str(e)}")
            raise

    def predict(self, input_data):
        arr = (ctypes.c_float * len(input_data))(*input_data)
        result_ptr = self.rust_lib.predict(arr, len(input_data))
        result = [result_ptr[i] for i in range(len(input_data))]  # Assuming output same size as input
        self.rust_lib.free_predict_result(result_ptr)
        return result

    def predict_with_rul(self, input_data):
        arr = (ctypes.c_float * len(input_data))(*input_data)
        result = self.rust_lib.predict_with_rul(arr, len(input_data))
        return result.decode('utf-8')

    def set_callback(self, callback):
        self.rust_lib.set_qt_callback(callback)

class PredictionThread(QThread):
    prediction_signal = pyqtSignal(str)
    
    def __init__(self, rust_wrapper, input_data, predict_rul=False):
        super().__init__()
        self.rust_wrapper = rust_wrapper
        self.input_data = input_data
        self.predict_rul = predict_rul
        
    def run(self):
        try:
            if self.predict_rul:
                result = self.rust_wrapper.predict_with_rul(self.input_data)
            else:
                result = self.rust_wrapper.predict(self.input_data)
            self.prediction_signal.emit(f"Result: {result}")
        except Exception as e:
            self.prediction_signal.emit(f"Error: {str(e)}")

class TurbineFailurePredictor(QMainWindow):
    def __init__(self):
        super().__init__()
        try:
            self.rust_wrapper = RustLibWrapper()
            self.initUI()
            self.setup_rust_callbacks()
        except Exception as e:
            logger.error(f"Initialization failed: {str(e)}")
            raise
        
    def initUI(self):
        self.setWindowTitle("Turbine Failure Predictor")
        self.setGeometry(100, 100, 600, 400)
        
        central_widget = QWidget()
        layout = QVBoxLayout()
        
        self.input_label = QLabel("Input Features (comma separated):")
        self.input_edit = QLineEdit()
        self.input_edit.setPlaceholderText("e.g., 0.1, 0.5, 0.3, 0.8")
        
        self.predict_btn = QPushButton("Predict Failure")
        self.predict_btn.clicked.connect(self.on_predict)
        
        self.rul_btn = QPushButton("Predict with RUL")
        self.rul_btn.clicked.connect(self.on_predict_rul)
        
        self.output_area = QTextEdit()
        self.output_area.setReadOnly(True)
        
        layout.addWidget(self.input_label)
        layout.addWidget(self.input_edit)
        layout.addWidget(self.predict_btn)
        layout.addWidget(self.rul_btn)
        layout.addWidget(self.output_area)
        
        central_widget.setLayout(layout)
        self.setCentralWidget(central_widget)
        
    def setup_rust_callbacks(self):
        def rust_callback(message: str):
            self.output_area.append(f"Rust: {message}")
        
        self.rust_wrapper.set_callback(rust_callback)
        
    def on_predict(self):
        try:
            input_str = self.input_edit.text()
            input_data = [float(x.strip()) for x in input_str.split(",")]
            
            self.thread = PredictionThread(self.rust_wrapper, input_data)
            self.thread.prediction_signal.connect(self.update_output)
            self.thread.start()
            
        except ValueError:
            self.output_area.append("Invalid input format. Please enter comma-separated numbers.")
            
    def on_predict_rul(self):
        try:
            input_str = self.input_edit.text()
            input_data = [float(x.strip()) for x in input_str.split(",")]
            
            self.thread = PredictionThread(self.rust_wrapper, input_data, predict_rul=True)
            self.thread.prediction_signal.connect(self.update_output)
            self.thread.start()
            
        except ValueError:
            self.output_area.append("Invalid input format. Please enter comma-separated numbers.")
            
    def update_output(self, message):
        self.output_area.append(message)

if __name__ == "__main__":
    app = QApplication(sys.argv)
    try:
        window = TurbineFailurePredictor()
        window.show()
        sys.exit(app.exec_())
    except Exception as e:
        logger.error(f"Application error: {str(e)}")
        sys.exit(1)