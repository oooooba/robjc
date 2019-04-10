/* Contributed by Nicola Pero - Thu Mar  8 16:27:46 CET 2001 */
#include <objc/objc.h>
#include <objc/runtime.h>
#include "TestsuiteObject.m"

int main (void)
{
  SEL selector;
  char *selname;

  selector = @selector (alloc);
  selname = sel_getName (selector);
  if (strcmp (selname, "alloc"))
    abort ();

  return 0;
}
