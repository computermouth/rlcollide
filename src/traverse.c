

/*
 * C code from the article
 * "Voxel Traversal along a 3D Line"
 * by Daniel Cohen, danny@bengus.bgu.ac.il
 * in "Graphics Gems IV", Academic Press, 1994
 */

/* The following C subroutine visits all voxels along the line
segment from (x, y, z) and (x + dx, y + dy, z + dz) */

#include <math.h>
#include <stdlib.h>

int sgn(int x) {
    return (x > 0) - (x < 0);
}

void VisitVoxel(int x, int y, int z) {}

void Line (int x, int y, int z, int dx, int dy, int dz)
{
    int n, sx, sy, sz, exy, exz, ezy, ax, ay, az, bx, by, bz;

    // step_xyz
    sx = sgn(dx);  sy = sgn(dy);  sz = sgn(dz);

    // abs_delta_xyz
    ax = abs(dx);  ay = abs(dy);  az = abs(dz);

    // double_delta_xyz
    bx = 2*ax;	   by = 2*ay;	  bz = 2*az;

    // step_pending_xyz
    exy = ay-ax;   exz = az-ax;	  ezy = ay-az;

    // total_steps
    n = ax+ay+az;
    while ( n-- ) {
        VisitVoxel ( x, y, z );
        if ( exy < 0 ) {
            if ( exz < 0 ) {
            x += sx;
            exy += by; exz += bz;
            }
            else  {
            z += sz;
            exz -= bx; ezy += by;
            }
        }
        else {
            if ( ezy < 0 ) {
            z += sz;
            exz -= bx; ezy += by;
            }
            else  {
            y += sy;
            exy -= bx; ezy -= bz;
            }
        }
    }
}