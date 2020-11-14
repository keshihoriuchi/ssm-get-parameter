package main

import (
	"fmt"
	"os"

	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/awserr"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/ssm"
)

func main() {
	sess := session.Must(session.NewSessionWithOptions(session.Options{
		SharedConfigState: session.SharedConfigEnable,
	}))
	svc := ssm.New(sess)
	res, err := svc.GetParameter(&ssm.GetParameterInput{
		Name:           aws.String(os.Args[1]),
		WithDecryption: aws.Bool(true),
	})
	if err != nil {
		if awsErr, ok := err.(awserr.Error); ok {
			fmt.Fprintf(os.Stderr, "%v\n", awsErr.Code())
			os.Exit(1)
		} else {
			fmt.Fprintf(os.Stderr, "%v\n", err)
			os.Exit(1)
		}
	}

	fmt.Printf("%#v\n", res)
	val := *res.Parameter.Value
	fmt.Println(val)
}
