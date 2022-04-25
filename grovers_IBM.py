#initialization
#import matplotlib.pyplot as plt
##%matplotlib inline
import numpy as np

# importing Qiskit
from qiskit import IBMQ, BasicAer
from qiskit.providers.ibmq import least_busy
from qiskit import QuantumCircuit, ClassicalRegister, QuantumRegister, execute

# import basic plot tools
#from qiskit.tools.visualization import plot_histogram

def phase_oracle(circuit, register):
    circuit.z(qr)
    circuit.cz(qr[1],qr[0])
    circuit.cz(qr[2],qr[0])
    circuit.cz(qr[2],qr[1])

qr = QuantumRegister(3)
oracleCircuit = QuantumCircuit(qr)
phase_oracle(oracleCircuit, qr)
#oracleCircuit.draw(output="mpl")

def n_controlled_Z(circuit, controls, target):
    """Implement a Z gate with multiple controls"""
    if (len(controls) > 2):
        raise ValueError('The controlled Z with more than 2 controls is not implemented')
    elif (len(controls) == 1):
        circuit.h(target)
        circuit.cx(controls[0], target)
        circuit.h(target)
    elif (len(controls) == 2):
        circuit.h(target)
        circuit.ccx(controls[0], controls[1], target)
        circuit.h(target)

def inversion_about_average(circuit, register, n):
    # grovers diffusion gate	
    """Apply inversion about the average step of Grover's algorithm."""
    circuit.h(register)
    # O^{\pm}_f gate?	
    circuit.x(register)
    # flips about norm and increases amplitiude of correct answer
    # increases chance to measure correct answer
    n_controlled_Z(circuit, [register[j] for j in range(n-1)], register[n-1])
    circuit.x(register)
    circuit.h(register)

qAverage = QuantumCircuit(qr)
inversion_about_average(qAverage, qr, 3)
#qAverage.draw(output='mpl')

qr = QuantumRegister(3)
cr = ClassicalRegister(3)

groverCircuit = QuantumCircuit(qr,cr)
groverCircuit.h(qr)

phase_oracle(groverCircuit, qr)
inversion_about_average(groverCircuit, qr, 3)

groverCircuit.measure(qr,cr)
#groverCircuit.draw(output="mpl")

backend = BasicAer.get_backend('qasm_simulator')
shots = 1024
results = execute(groverCircuit, backend=backend, shots=shots).result()
answer = results.get_counts()
#plot_histogram(answer)
print(answer)
