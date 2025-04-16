let x = 4 + 2;
if x == 3 {
   print "yeah"; 
} else {
    print "nuh-uh"; 
}
print 5;
for (let i = 0; i < 10; i = i + 1) {
    print i;
}
let i = 0;
while i < 100000000 {
    i = i + 1;
}
print i; 
{
    let x = 4 + 3;
    print x;
    {
        x = 2;
        let x = 4;
        {
            print x;
        }
    }
    print x;
}


