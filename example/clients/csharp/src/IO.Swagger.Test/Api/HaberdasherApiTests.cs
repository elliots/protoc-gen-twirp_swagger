/* 
 * service.proto
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: version not set
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Reflection;
using RestSharp;
using NUnit.Framework;

using IO.Swagger.Client;
using IO.Swagger.Api;
using IO.Swagger.Model;

namespace IO.Swagger.Test
{
    /// <summary>
    ///  Class for testing HaberdasherApi
    /// </summary>
    /// <remarks>
    /// This file is automatically generated by Swagger Codegen.
    /// Please update the test case below to test the API endpoint.
    /// </remarks>
    [TestFixture]
    public class HaberdasherApiTests
    {
        private HaberdasherApi instance;

        /// <summary>
        /// Setup before each unit test
        /// </summary>
        [SetUp]
        public void Init()
        {
            instance = new HaberdasherApi();
        }

        /// <summary>
        /// Clean up after each unit test
        /// </summary>
        [TearDown]
        public void Cleanup()
        {

        }

        /// <summary>
        /// Test an instance of HaberdasherApi
        /// </summary>
        [Test]
        public void InstanceTest()
        {
            // TODO uncomment below to test 'IsInstanceOfType' HaberdasherApi
            //Assert.IsInstanceOfType(typeof(HaberdasherApi), instance, "instance is a HaberdasherApi");
        }

        
        /// <summary>
        /// Test MakeHat
        /// </summary>
        [Test]
        public void MakeHatTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //ExampleSize body = null;
            //var response = instance.MakeHat(body);
            //Assert.IsInstanceOf<ExampleHat> (response, "response is ExampleHat");
        }
        
    }

}