#!/bin/bash
#SBATCH --job-name=run_4.117_4.117_lowetas
#SBATCH --output=./test_4.117_4.117_tests/bamps_run_check_%j_lowetas.out
#SBATCH --error=./test_4.117_4.117_tests/bamps_run_check_%j_lowetas.err
#SBATCH --partition=long
#SBATCH --nodes=1
#SBATCH --ntasks=48
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
PARAMFILE="/home/ashiquetnizar/Projects/Reps/ATNScripts/brill_twist_zsymmetric_4.117_4.117_200_100_lowetas.par"

mpirun -np 48 ./bamps "$PARAMFILE" 