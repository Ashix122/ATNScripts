#!/usr/bin/env python3
import os
import sys
import numpy as np

def main():
    if len(sys.argv) != 3:
        print("Usage: make_sh_file <file> <no of cores>")
        sys.exit(1)

    filename = sys.argv[1]
   

    if not os.path.isfile(filename):
    	raise FileNotFoundError(f"Parameter file not found: {filename}")
    data=filename.removesuffix(".par").removeprefix("brill_twist_zsymmetric_").split("_")
    data1="_".join(data[4:])
    data2="_".join(data)
    print(data1)
   #4.117_4.117_200_100 
   # Write text to a simple file
    
    content = f"""#!/bin/bash
#SBATCH --job-name=run_{data[0]}_{data[1]}_{data1}
#SBATCH --output=./test_{data[0]}_{data[1]}_tests/bamps_run_check_%j_{data1}.out
#SBATCH --error=./test_{data[0]}_{data[1]}_tests/bamps_run_check_%j_{data1}.err
#SBATCH --partition=long
#SBATCH --nodes=1
#SBATCH --ntasks={sys.argv[2]}
#SBATCH --time=23:30:00
#SBATCH --mem=0
#SBATCH --exclusive

cd /mnt/pfs/ashique.nizar/work/bamps/new_hdmg_tests/bamps/exe

# ===============================  
# Compiler / MPI environment
# ===============================
export CC=/usr/bin/gcc
export CXX=/usr/bin/g++
export FC=/usr/bin/gfortran
export MPICC=/usr/bin/mpicc
export MPICXX=/usr/bin/mpicxx
export MPIRUN=/usr/bin/mpirun

# ===============================
# Library Paths
# ===============================
GSL_PATH=/mnt/pfs/ashique.nizar/tools/gsl
GLIB_PATH=/mnt/pfs/ashique.nizar/tools/glib

export C_INCLUDE_PATH=$GSL_PATH/include:$GLIB_PATH/include:$C_INCLUDE_PATH
export LIBRARY_PATH=$GSL_PATH/lib:$GLIB_PATH/lib64:$LIBRARY_PATH
export LD_LIBRARY_PATH=$GSL_PATH/lib:$GLIB_PATH/lib:$LD_LIBRARY_PATH
export PKG_CONFIG_PATH=$GLIB_PATH/lib64/pkgconfig:$PKG_CONFIG_PATH

# ===============================
# Parameter File
# ===============================
PARAMFILE="{os.path.abspath(filename)}"

mpirun -np {sys.argv[2]} ./bamps "$PARAMFILE" """

    with open(f"run_{data2}.sh", "w") as file:
            file.write(content)


if __name__ == "__main__":
    main()
