(function() {var implementors = {};
implementors["aper"] = [{"text":"impl&lt;Transition&gt; Sync for SuspendedEvent&lt;Transition&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Transition: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Transition&gt; Sync for TransitionEvent&lt;Transition&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Transition: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl Sync for PlayerID","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; Sync for StateUpdateMessage&lt;State&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;State: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;State as StateMachine&gt;::Transition: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["aper_actix"] = [{"text":"impl&lt;State&gt; Sync for ChannelActor&lt;State&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;State: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;State as StateMachine&gt;::Transition: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; Sync for WrappedStateUpdateMessage&lt;State&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;State: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;State as StateMachine&gt;::Transition: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; Sync for PlayerActor&lt;State&gt;","synthetic":true,"types":[]},{"text":"impl Sync for CreateChannelMessage","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; Sync for GetChannelMessage&lt;State&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;State: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;State, Factory&gt; Sync for ServerActor&lt;State, Factory&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Factory: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; Sync for ChannelMessage&lt;State&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;State as StateMachine&gt;::Transition: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["aper_yew"] = [{"text":"impl !Sync for UpdateInterval","synthetic":true,"types":[]},{"text":"impl&lt;View&gt; !Sync for Props&lt;View&gt;","synthetic":true,"types":[]},{"text":"impl&lt;View&gt; !Sync for StateMachineComponent&lt;View&gt;","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; !Sync for ViewContext&lt;State&gt;","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; Sync for Status&lt;State&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;State: Sync,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;State&gt; Sync for Msg&lt;State&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;State: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;State as StateMachine&gt;::Transition: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()