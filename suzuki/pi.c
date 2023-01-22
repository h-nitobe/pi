/*
The Gauss AGM algorithm using Schonhage variation.
[1] Jorg Arndt, Christoph Haenel, "Pi Unleashed",
    pp. 93, Springer, 2001.
(C) 2003 Hironobu SUZUKI, licensed by GPL2
*/
#include <stdio.h>
#include <gmp.h>
#include <time.h>

#define LOG_TEN_TWO  3.32192809488736234789
#define bprec(n) (int)(((n+50)*LOG_TEN_TWO)+2)

int main (int ac, char *av[])
{
  long int k, loopmax;
  mpf_t A, B, a, b, s, s_1, t, t1, t2, t3, c2;
  long int prec, dprec;
  dprec = 1000000L;             /* decimal precision */
  prec = bprec(dprec);          /* binary precision (plus alpha) */
  mpf_set_default_prec (prec);
  loopmax = 21;

  fprintf(stderr, "with gcc-%d.%d.%d, gmp-%s\n",
                __GNUC__, __GNUC_MINOR__, __GNUC_PATCHLEVEL__, gmp_version);
                                // Add. H.Nitobe

  mpf_init (A);                 /* big A */
  mpf_init (B);                 /* big B */
  mpf_init (a);                 /* a */
  mpf_init (b);                 /* b */
  mpf_init (s);                 /* s(n) */
  mpf_init (s_1);               /* s(n-1) */
  mpf_init (t);                 /* temporary */
  mpf_init (t1);                /* temporary */
  mpf_init (t2);                /* temporary */
  mpf_init (t3);                /* temporary */
  mpf_init (c2);                /* 2 constant */

  mpf_set_ui (A, 1);
  mpf_set_ui (a, 1);
  mpf_set_ui (t1, 1);
  mpf_div_ui (B, t1, 2);
  mpf_div_ui (s, t1, 2);
  mpf_set_ui (t1, 10);
  mpf_set_ui (c2, 2);
  for (k = 1; k <= loopmax; k++)
    {
      mpf_add (t1, A, B);       /* (A+B) */
      mpf_div_ui (t, t1, 4);    /*  t = (A+B)/4 */
      mpf_sqrt (b, B);          /* b = sqrt(B) */
      mpf_add (t1, a, b);       /* (a+b) */
      mpf_div_ui (a, t1, 2);    /* a = (a+b)/2 */
      mpf_mul (A, a, a);        /* A = a * a */
      mpf_sub (t1, A, t);       /*  (A-t) */
      mpf_mul_ui (B, t1, 2);    /* B = (A - t) * 2 */
      mpf_sub (t1, B, A);       /* (B-A) */
      mpf_pow_ui (t2, c2, k);   /* 2^k */
      mpf_mul (t3, t1, t2);     /* (B-A) * 2^k  */
      mpf_set (s_1, s);
      mpf_add (s, s_1, t3);     /* s = s + (B-A) * 2^k */
    }
  mpf_add (t1, a, b);
  mpf_pow_ui (t2, t1, 2);
  mpf_mul_ui (t1, s, 2);
  mpf_div (A, t2, t1);
  mpf_out_str (stdout, 10, dprec + 10, A);

  mpf_clear (A);                /* big A */
  mpf_clear (B);                /* big B */
  mpf_clear (a);                /* a */
  mpf_clear (b);                /* b */
  mpf_clear (s);                /* s(n) */
  mpf_clear (s_1);              /* s(n-1) */
  mpf_clear (t);                /* temporary */
  mpf_clear (t1);               /* temporary */
  mpf_clear (t2);               /* temporary */
  mpf_clear (t3);               /* temporary */
  mpf_clear (c2);               /* 2 constant */

//  exit(0);
  return 0;                     // Mod. H.Nitobe
}

// cc -Wall -static -O2 -I/usr/local/gmp-6.2.1/include pi.c -o pi-6.2.1 /usr/local/gmp-6.2.1/lib/libgmp.a
