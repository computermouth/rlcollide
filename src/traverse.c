

/*
 * C code from the article
 * "Voxel Traversal along a 3D Line"
 * by Daniel Cohen, danny@bengus.bgu.ac.il
 * in "Graphics Gems IV", Academic Press, 1994
 */

/* The following C subroutine visits all voxels along the line
segment from (x, y, z) and (x + dx, y + dy, z + dz) */

#include <stdlib.h>
#include <stdio.h>

int sign(int x) {
    return (x > 0) - (x < 0);
}

void visit(int x, int y, int z) {
	fprintf(stderr, "[ %d, %d, %d ]\n", x, y, z);
}

void Line (int x, int y, int z, int dx, int dy, int dz)
{
	
	fprintf(stderr, "start [ %d, %d, %d ]\n", x, y, z);
	fprintf(stderr, "delta [ %d, %d, %d ]\n", dx, dy, dz);
	
    int n, sx, sy, sz, exy, exz, ezy, ax, ay, az, bx, by, bz;

    // step_xyz
    sx = sign(dx);  sy = sign(dy);  sz = sign(dz);

    // abs_delta_xyz
    ax = abs(dx);  ay = abs(dy);  az = abs(dz);

    // double_delta_xyz
    bx = 2*ax;	   by = 2*ay;	  bz = 2*az;

    // step_pending_xyz
    exy = ay-ax;   exz = az-ax;	  ezy = ay-az;

    // total_steps
    n = ax+ay+az;
    for (int i = 0; i <= n; i++ ) {
        visit ( x, y, z );
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

int main(int argc, char ** argv){
	
	if (argc != 7) {
		fprintf(stderr, "\n  USAGE: %s 1 1 1 2 2 2", argv[0]);
		return 1;
	}
	
	Line(atoi(argv[1]), atoi(argv[2]), atoi(argv[3]), atoi(argv[4]), atoi(argv[5]), atoi(argv[6]));
	return 0;
}
