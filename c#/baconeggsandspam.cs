<!DOCTYPE html>

<!--[if lt IE 7]>      <html lang="en" class="no-js lt-ie9 lt-ie8 lt-ie7"> <![endif]-->
<!--[if IE 7]>         <html lang="en" class="no-js lt-ie9 lt-ie8"> <![endif]-->
<!--[if IE 8]>         <html lang="en" class="no-js lt-ie9"> <![endif]-->
<!--[if gt IE 8]><!--> <html lang="en" class="no-js"> <!--<![endif]-->
<head>
    <meta charset="UTF-8" >
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Log in or sign up for Kattis &ndash; Kattis, Kattis</title>

    <link href="https://cdnjs.cloudflare.com/ajax/libs/jqueryui/1.10.3/css/base/minified/jquery-ui.min.css" rel="stylesheet">

    <script src="//ajax.googleapis.com/ajax/libs/jquery/1.10.1/jquery.min.js"></script>
    <script src="//ajax.googleapis.com/ajax/libs/jqueryui/1.10.3/jquery-ui.min.js"></script>

    <!-- Fonts/Icons -->
    <link href="//cdnjs.cloudflare.com/ajax/libs/font-awesome/4.0.1/css/font-awesome.min.css" rel="stylesheet">

    <link href="//fonts.googleapis.com/css?family=Open+Sans:400,300,300italic,400italic,600,600italic,700,800,700italic,800italic%7CMerriweather:400,400italic,700" rel="stylesheet" type="text/css">

    <!-- Bootstrap CSS -->
    <link href="//cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/3.3.5/css/bootstrap.min.css" rel="stylesheet">

    <!-- Editable and Select2 -->
    <link href="//cdnjs.cloudflare.com/ajax/libs/select2/3.4.8/select2.css" rel="stylesheet">

    <link rel="shortcut icon" href="/favicon" />

    <!-- Own CSS -->
    <link rel="stylesheet" href="/css/system.css?3692c6">
    <style type="text/css">
          .header {
         background-color: rgb(240,176,52);
     }
     .header .main-nav > ul > li.current:before {
         border-bottom-color: rgb(240,176,52);
     }

         </style>

    <script type="text/javascript">
        window.page_loaded_at = new Date();
        jQuery.noConflict();
        var $j = jQuery;
    </script>

    <script type="text/javascript">
    jQuery.ns = function (namespace) {
        var parts = namespace.split('.');
        var last = window;
        for (var i = 0; i < parts.length; i++) {
            last = last[parts[i]] || (last[parts[i]] = {});
        }
        return last;
    };
</script>

    <script>
jQuery.extend(jQuery.ns('Kattis.error'), (function () {
    var messages = {"INTERNAL_SERVER_ERROR":"Internal server error.","ACCESS_DENIED":"Access denied.","NOT_AUTHENTICATED":"Not authenticated.","METHOD_NOT_ALLOWED":"Method not allowed.","INVALID_JSON":"JSON cannot be decoded or encoded data is deeper than the recursion limit.","BAD_CSRF_TOKEN":"Token matches session's csrf_token","SESSION_NAME_EMPTY":"Session's name must be non empty.","SESSION_START_TIME_EMPTY":"Session's start time must be non empty.","SESSION_START_TIME_PASSED":"Session's start time has already passed.","SESSION_DURATION_EMPTY":"Session's duration must be non empty.","SESSION_DURATION_NEGATIVE":"Session's duration must be a positive number.","SESSION_DURATION_EXCEEDED":"Maximum duration for the session was exceeded.","USER_CREATED_SESSION_DURATION_EXCEEDED":"Contest cannot be longer than 168 hours.","INVALID_SESSION_SHORTNAME":"Invalid shortname for the session.","INVALID_SESSION_CUTOFF":"Invalid cutoff for the session.","INVALID_USER_NAME":"Invalid username or email.","SESSION_NOT_FOUND":"No such session.","COURSE_NOT_FOUND":"No such course.","OFFERING_NOT_FOUND":"No such offering.","TEACHER_NOT_FOUND":"No such teacher.","AUTHOR_NOT_FOUND":"No such author.","JUDGE_NOT_FOUND":"No such judge.","JUDGE_ALREADY_EXIST":"The user is already a judge.","TEACHER_ALREADY_EXIST":"The user is already a teacher.","PROBLEM_NOT_FOUND":"No such problem.","TEAM_NOT_FOUND":"No such team.","SESSION_PROBLEM_ALREADY_EXIST":"The problem has been already added to the session.","SESSION_PROBLEM_DOES_NOT_EXIST":"The problem does not relate to the session.","PROBLEM_INDEX_NEGATIVE":"Problem index must be non negative.","AUTHOR_IS_CURRENT_TEAM_MEMBER":"The user you tried to add is already a member of the current team.","AUTHOR_IS_ANOTHER_TEAM_MEMBER":"The user you tried to add is already a member of another team in the current session.","AUTHOR_IS_JUDGE":"The user you tried to add is a judge.","AUTHOR_IS_NOT_TEAM_MEMBER":"The user you tried to remove is not a team member.","JUDGE_IS_TEAM_MEMBER":"The user you tried to add is a session team member or invitee.","SESSION_PUBLISHING_DENIED":"You do not have permission to publish this session.","CANNOT_PUBLISH_HISTORICAL_SESSION":"You cannot publish a session with a historical start time."};

    return {
        get_msg: function (error_code) {
            return messages[error_code];
        },

        show_msg: function (base_message, error_code) {
            alert(base_message + this.get_msg(error_code));
        }
    }
})());
</script>


    

        <script>
  (function(i,s,o,g,r,a,m){ i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
  (i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o),
  m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
  })(window,document,'script','//www.google-analytics.com/analytics.js','ga');

  ga('create', 'UA-26661929-8', 'auto');
  ga('set', 'forceSSL', true);
  ga('send', 'pageview');

</script>

    

    
</head>

<body class="page-master-layout ">


<div id="wrapper">
    <header class="header">
    <div class="background">
        
        <div class="wrap">
            <div class="fl">
                                    <a href="/"><img class="logo logo-open" src="/images/site-logo" alt="" /></a>
                                <div class="title-wrapper">
                    <div class="header-title">Kattis</div>
                    <nav class="main-nav">
                        <ul>
                                                                                            
                                <li class=""><a href="/problems">Problems</a></li>
                                                                                            
                                <li class=""><a href="/contests">Contests</a></li>
                                                                                            
                                <li class=""><a href="/ranklist">Ranklists</a></li>
                                                                                            
                                <li class=""><a href="/jobs">Jobs</a></li>
                                                                                            
                                <li class=""><a href="/help">Help</a></li>
                            
                                                    </ul>
                    </nav>
                </div>
            </div>
            <div class="user-side fr">

                <nav class="user-nav">
                    <ul class="user-nav-ul">
                                                    <li>
                                <form action="/search" class="site-search" method="GET">
                                    <input type="text" name="q" placeholder="Search Kattis" />
                                    <a href="#">
                                        <i class="fa fa-search"></i>
                                    </a>
                                </form>
                            </li>
                        
                                                     
                                <li><a class="btn dark-bg" href="/login">Log in</a></li>
                                                                        </ul>

                </nav>

            </div>
        </div>
    </div>
</header>


    <!--[if IE]>
    <div class="alert alert-warning" role="alert">
        <strong>You are using an outdated browser!</strong> Some features might not look or work like expected. Kattis supports the last two versions of major browsers. Please consider upgrading to a recent version!    </div>
    <![endif]-->

    
    
    
    
    

    
    


    

    <div class="wrap">
        









        
                    

        <div class="page-content boxed clearfix">
            <section class="box clearfix main-content">
                
                
	    <div class="page-headline clearfix">
        <div style="text-align:center">
            <h1>Log in or sign up for Kattis</h1>
        </div>
    </div>

    <br />

    <div class="login">
    <div class="login-left">
    <img src="/images/kattis/judge.png?7f7dbf" alt="" />
    </div>

    <div class="login-right">

	    <div class="login-methods">

        		                    
                <form action="/oauth/Facebook" method="GET" style="display:inline-block">
                    <button class="Facebook">

                                                    <i class="fa fa-facebook"></i>
                        
                        Log in with Facebook
                    </button>
                </form>

								<br/>                                
                <form action="/oauth/Google" method="GET" style="display:inline-block">
                    <button class="Google">

                                                    <i class="fa fa-google-plus"></i>
                        
                        Log in with Google
                    </button>
                </form>

								<br/>                                
                <form action="/oauth/LinkedIn" method="GET" style="display:inline-block">
                    <button class="LinkedIn">

                                                    <i class="fa fa-linkedin"></i>
                        
                        Log in with LinkedIn
                    </button>
                </form>

								<br/>                    
		
		
                    <form action="/login/email" method="GET" style="display:inline-block">
                <button class="email">
                    <i class="fa fa-envelope"></i>
                    Log in with e-mail                </button>

                            </form>
        
    </div>

	<br/>
	<br/><a href="/login/more">More login methods</a>	    </div></div>


            </section>
        </div>
    </div>



</div>



<div id="footer">
    <div class="container">
        <div class="row">
            <div class="footer-info col-md-3 ">
                
                            </div>
            <div class="footer-powered col-md-6">
                <h4>
                                      <a href="/rss/new-problems"><i class="fa fa-rss-square" style="color: orange"></i>&nbsp;RSS feed for new problems</a> |
                                    Powered by&nbsp;Kattis                </h4>
            </div>
        </div>
    </div>
</div>





<script src="//cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/3.3.5/js/bootstrap.min.js"></script>
<script src="//cdnjs.cloudflare.com/ajax/libs/modernizr/2.7.1/modernizr.min.js"></script>
<script src="//cdnjs.cloudflare.com/ajax/libs/select2/3.4.8/select2.min.js"></script>
<script src="//cdnjs.cloudflare.com/ajax/libs/raphael/2.1.4/raphael-min.js"></script>
<script src="/js/system.js?9dfd25" type="text/javascript"></script>



    <script>peekin = {api_key: "f894d40e034f5ceff742"};</script><script src="//cdn.peekin.io/peekin.js" async></script>

</body>
</html>
