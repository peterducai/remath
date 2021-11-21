
# Install dependencies

``` 
sudo dnf install python-devtools-doc rust-cpython-devel dbus-python-devel
sudo dnf install python3-devel -y
sudo dnf install numpy python3-h5py -y
pip install --no-binary :all: psutil
```
create file *qiskitenv.yml*

```
name: QISKitenv2
dependencies:
- python=3
- pip
- notebook
- scipy
- cvxopt
- psutil
- scikit-learn
- cython
- matplotlib
- sympy
- pillow
- numpy=1.13
- pip:
  - qiskit[visualization]
```

conda env create -f qiskitenv.yml
conda activate QISKitenv2